use crate::JsonRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, RefactorKind, Rule, RuleAction,
    RuleDiagnostic, SourceActionKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_json_factory::make::{json_member_list, token};
use biome_json_syntax::{JsonMember, JsonMemberList, JsonMemberName, JsonRoot, JsonSyntaxToken, T};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt, TextRange, TextSize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: red;
    /// }
    /// ```
    ///
    pub UseSortedKeys {
        version: "next",
        name: "useSortedKeys",
        recommended: false,
    }
}

#[derive(Eq, PartialEq)]
pub struct MemberKey {
    node: JsonMember,
}

impl MemberKey {
    fn range(&self) -> TextRange {
        self.node.range()
    }
}

impl Ord for MemberKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort imports using natural ordering
        natord::compare(
            &self.node.name().unwrap().text(),
            &other.node.name().unwrap().text(),
        )
    }
}

impl PartialOrd for MemberKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Members(pub BTreeSet<MemberKey>);

impl Members {
    /// Returns true if the nodes in the group are already sorted in the file
    fn is_sorted(&self) -> bool {
        // The imports are sorted if the text position of each node in the `BTreeMap`
        // (sorted in natural order) is higher than the previous item in
        // the sequence
        let mut iter = self
            .0
            .iter()
            .map(|node| node.node.syntax().text_range().start());
        let mut previous_start = iter.next().unwrap_or_default();
        iter.all(|start| {
            let is_sorted = previous_start < start;
            previous_start = start;
            is_sorted
        })
    }

    fn to_sorted_node(&self) -> JsonMemberList {
        let items = self.0.iter().map(|key| key.node.clone().detach());

        let separator_count = items.len().saturating_sub(1);

        let mut separators = Vec::new();

        for (index, _) in self.0.iter().enumerate() {
            if index == separator_count {
                continue;
            } else {
                separators.push(token(T![,]))
            }
        }

        let member_list = json_member_list(items, separators);

        member_list
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsonMemberList>;
    type State = Members;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.is_empty() {
            return None;
        }

        let state = node
            .iter()
            .filter_map(|node| {
                let node = node.ok()?;
                Some(MemberKey { node })
            })
            .collect::<BTreeSet<_>>();

        let state = Members(state);

        if !state.is_sorted() {
            Some(state)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The key can be sorted"
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsonRuleAction> {
        let list = state.to_sorted_node();
        let mut mutation = ctx.root().begin();
        let node = ctx.query().clone();
        mutation.replace_node(node, list);

        Some(RuleAction {
            mutation,
            category: ActionCategory::Refactor(RefactorKind::Rewrite),
            applicability: Applicability::MaybeIncorrect,
            message: markup! {
                "Sort the keys"
            }
            .to_owned(),
        })
    }
}
