# Training Exchange: design trace

This is a hand-worked trace against [the draft schema](schema-draft.md) and
[the complete encounter note](examples/campaign/Training%20Exchange.md). No
simulator was run. Listed dice are controlled example inputs, not a claim that
a particular seed generates them. Runtime labels below stand for independent
UUIDs; no runtime record is written to an authored note.

## Preparation

Resolve the four roster references to one Warden definition and instantiate
four separate actors, inventories, pools and supply packs. Select D&D 2024 core
rules. The actual app cannot run this document until those contracts exist.
The manual controllers supply the decisions below. Read the encounter's stated
reach, sight and willingness assumptions before interpreting any targeting.

| Actor | Side | Initial HP / maximum | Initiative die + modifier | Runtime order |
| --- | --- | --- | --- | --- |
| Cinder | Red | 30 / 30 | 18 + 2 = 20 | 1 |
| Aster | Blue | 30 / 30 | 14 + 2 = 16 | 2 |
| Briar | Blue | 18 / 30 | 10 + 2 = 12 | 3 |
| Dusk | Blue | 30 / 30 | 6 + 2 = 8 | 4 |

Every actor starts with AC 16; the shield is carried. Each has three Healing
Uses, two level-one slots, three carried draughts, and independent wand pools.
The worn pack has a nested pouch and two additional stored draughts. No row
identity is shared across these instances, even though row keys are identical.

## Round one

### Cinder casts Ember Wave

1. Resolve actor action alias `ember_wave` to the spell's `cast`. Bind
   `slot_pool=spell_slots_1`, `cast_level=1`, `save_dc=13`. Keep both source
   addresses on use `U1`.
2. Select Aster, Briar and Dusk. Validate target count, range and visibility.
   Cinder has a free hand for the somatic component and can supply the verbal
   component. This example has no hidden invalid target.
3. Commit the action and begin the rule-managed casting operation. Open its
   casting reaction opportunities. None of these participants can counterspell,
   so normal casting expends one level-one slot and records this turn's slot
   expenditure. Cinder's slots become 1. This ordering in the trace is not a
   generic 'spend then refund' rule for every casting interruption.
4. Roll `U1/damage` once: 4 + 5 = 9 fire damage. Its scope is the use, so all
   targets share the same roll record.
5. Resolve separate Dexterity saves against DC 13: Aster 4 + 2 = 6 fails,
   Briar 15 + 2 = 17 succeeds, Dusk 3 + 2 = 5 fails. Apply damage 9, 4 and 9.
   Each target gets a separate damage event linked to the shared use/roll.
6. HP becomes Aster 21, Briar 14, Dusk 21. Protect Ally is not eligible: these
   are save-based damage events, not pending attack hits. No Concentration
   checks are needed because nobody is concentrating.

### Aster heals and drinks a draught

1. Start Aster's turn and refresh ordinary budgets according to core rules.
   Select willing Briar for `mend`. The touch assumption is explicit.
2. Commit one action and one Healing Use (3 → 2). Roll 6 + 2 = 8. Briar's HP
   changes 14 → 22. Finish use `U2`; the roll is not an ongoing effect.
3. Use the carried draught stack's `drink` action with Aster as target. Its
   `source_item` is Aster's carried `potions` stack, not the two stored reserve
   draughts. Aster has one free hand; access is valid.
4. Commit the Bonus Action. At `resolution`, consume one draught (3 → 2), then
   roll 2 + 3 + 2 = 7 healing. Aster's HP changes 21 → 28.

The action and Bonus Action are separate uses with separate roll/cost records.
The stored reserve stack and every other participant's draughts are unchanged.

### Briar uses Wand A

1. Start Briar's turn. Use the core allowed object interaction to draw Wand A
   into the free hand. Its equipment changes `carried` → `wielded`; the spear
   remains in the other hand. No shield is wielded. The declared interaction
   must be legal under the selected core rule, not free because it is a table edit.
2. Select Wand A's `flare` and target Cinder. Bind its exact runtime item as
   `source_item`. Commit one action and one fire charge (4 → 3).
3. Cinder's Dexterity save is 3 + 2 = 5 against DC 12: failure. The damage roll
   is 4; Cinder's HP changes 30 → 26. There is no attack hit and no Protect
   Ally opportunity. Wand A's healing pool stays 2; Wand B stays at 1/1.

### Dusk heals Briar

Start Dusk's turn, select Briar, and commit Mend's action and one Healing Use
(3 → 2). Roll 4 + 2 = 6. Briar's HP changes 22 → 28. Dusk's own HP remains 21.

## Round two: pending hit and reaction

1. Cinder selects Briar for Strike. The spear is wielded, reach is declared,
   and Cinder commits the action. Create attack resolution `A1` under use `U6`.
2. Roll 12 + 5 = 17 against AC 16. It is a provisional noncritical hit.
   Emit `dnd2024:attack.hit_pending` with target Briar and resolution `A1`.
   Damage has not yet been rolled.
3. Aster and Dusk qualify for Protect Ally. Briar cannot protect itself.
   Cinder's controller chooses Aster first under the simultaneous-effects rule
   for this turn. That choice happens to match the stable fallback order; the
   fallback does not override the controller's rule-given choice.
4. Aster accepts. Recheck current eligibility, then spend Aster's reaction.
   Child use `U7` creates one effect application on Briar. It has stacking key
   `my_homebrew:protect_ally`, origin Aster's use, and expiry `A1` ending.
   The modifier applies only when evaluating AC for `A1`.
5. Re-evaluate the existing attack total 17 against AC 18. It is now a miss.
   Dusk's opportunity fails revalidation; Dusk spends no reaction. Do not
   reroll the attack or offer Aster the same event again.
6. Finish `A1` without rolling on-hit damage. Expire the ward even though the
   attack missed. Briar's effective AC returns to 16 for later attacks.
   Cinder's committed action remains spent.

If Aster had declined, Dusk could have chosen to react. If the original attack
die had been a natural 20, +2 AC would not remove the ordinary critical hit.
Those are separate cases; they do not alter this trace's recorded decisions.

## Manual ending and final observations

After Cinder's turn, end the encounter manually with outcome `undecided`. The
end reason is `manual`, not a victory condition and not a simulation failure.

| Actor | HP | Healing Uses | Level-one slots | Carried draughts | Changes to item state |
| --- | --- | --- | --- | --- | --- |
| Aster | 28 | 2 | 2 | 2 | None beyond consumed draught |
| Briar | 28 | 3 | 2 | 3 | Wand A wielded, fire 3 / healing 2; Wand B still 1 / 1 |
| Cinder | 26 | 3 | 1 | 3 | None |
| Dusk | 21 | 2 | 2 | 3 | None |

Aster has spent its reaction until the applicable refresh; Dusk has not. There
are no surviving ward effects. Every supply pack, pouch and stored reserve
stack is unchanged. The Warden note still says three Healing Uses and its
original inventory quantities. A new independent run starts from authored
initial state, including Briar's 18 HP, not this table.

## Checks exposed by the trace

- External aliases must bind resource identities as well as scalar parameters.
- Rolls need a scope owner; one damage roll does not mean one target outcome.
- Target selection, cast commitment and hidden invalidity must be separate.
- An attack's provisional outcome must support rule-driven re-evaluation.
- Costs and economy cannot be rolled back merely because the final attack misses.
- A reaction effect needs an exact resolution ID as its lifetime and stat scope.
- Equipment transitions and item-source binding need runtime identity.
- Rule costs can occur at commitment or resolution without being copied into prose.
- A seed alone cannot reproduce the manual decisions shown here.

## Transfer and interruption branches

These are additional controlled branches, not later events silently added to
the trace. They exercise the same records without changing its final table.

**Transfer:** with access and capacity requirements satisfied, transfer Briar's
existing pack to Aster. One state operation changes its immediate holder; its
item ID and child pouch/reserve IDs stay the same. Aster now has two distinct
packs with identical source row names. Runtime IDs keep them distinct. Capacity
or access rejection leaves the holder and contents unchanged; any already paid
interaction cost follows its own rule. The source notes remain unchanged.

**Counterspell:** add an explicitly defined eligible counterspell actor in an
Arcane variant. If its interruption rule cancels Ember Wave, apply the revised
rule that the spell slot is not expended, and retain the wasted casting action.
See [Action resolution](action-resolution.md) for its trigger and save. This
is not a generic rollback of use `U1` or a restoration of every other cost.

**Missing assumption:** remove the sight fact for Cinder and Briar. Preparing a
cast that needs it reports unresolved visibility rather than assuming it true.
No geometry engine is introduced to fill the gap.
