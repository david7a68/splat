# Action resolution under the 2024 rules

Status: design contract grounded in the official D&D 2024 rules and SRD 5.2.1.
The user's direction is to match those rules. This takes precedence over an
earlier generic timing or cost proposal where the two differ. It does not
authorize implementation. [The schema](schema-draft.md) supplies data shapes;
the game rule determines what each operation does.

## Authority and scope

Use the general game rule unless a particular feature supplies an exception.
An engine event is a way to represent that rule, not a new permission to act.
Every rule operation must record a primary source and identify any explicit
exception it applies. Homebrew may define its own behavior, but remains labeled
homebrew. The Test Warden's Protect Ally is not the official Shield spell.

The following distinctions are required:

| Concept | Meaning |
| --- | --- |
| Trigger occurrence | Something the game says happened and to which a feature can respond |
| Response timing | The feature's permitted point before, during or after that occurrence |
| Current eligibility | Whether the actor can use this feature now, including its available reaction and source-specific requirements |
| Pending result | A result a specific rule still permits changing, such as a hit affected by Shield |
| Completed result | A result that remains in history unless another rule explicitly changes it |

Do not universally undo an earlier trigger because later state differs. Do not
universally open reactions at every internal step. Refreshing cached state,
rechecking a target or emitting a diagnostic is not itself a game event.

## Reactions and timing

The general rule is that a Reaction occurs immediately after its trigger,
unless its description says otherwise. A Reaction can occur on the actor's
own turn or someone else's. After using an ordinary Reaction, the actor cannot
use another until the start of its next turn. Incapacitated forbids Reactions.

| Feature | Required timing | Consequence for resolution |
| --- | --- | --- |
| Ordinary Reaction without a timing exception | Immediately after its trigger | Retain the completed trigger; resolve the response at that boundary |
| Ready | Immediately after the chosen perceivable trigger finishes | The user can respond or ignore it; do not interrupt the trigger midway |
| Opportunity Attack | Immediately before the provoking creature leaves reach | Movement is interrupted at the specified boundary, not rolled back from a completed departure |
| Shield | On being hit by an attack or targeted by Magic Missile, as specified by the spell | Its AC bonus includes the triggering attack and persists until the start of the caster's next turn; do not reduce it to a one-attack bonus |
| Counterspell | While the observed creature casts a qualifying spell | Resolve the interruption before that spell takes effect |
| Test Warden's Protect Ally | Homebrew pending-hit opportunity before damage | Its authored +2 AC applies only to that attack; later opportunities require that pending hit still to qualify |

When one Reaction triggers another valid Reaction, resolve the child at its
specified timing before resuming the suspended resolution. The child's trigger
must actually occur and its actor must have a permitted use available. A generic
stack of actions is not permission to respond with any ability at any time.

Retain the event ID, feature instance, reacting actor, parent resolution and
choice. Duplicate processing must not offer the same opportunity again.
Distinct future occurrences remain distinct even if their display text matches.
Application of a source-specific repeat rule is separate from duplicate delivery.

## Ordering simultaneous effects

When two or more things occur at the same time on a turn, the person whose
turn it is decides their order. In manual play this is that turn's controller;
in simulation its configured strategy makes the permitted choice. Do not
replace the game-given choice with a mandatory initiative sort.

The already agreed initiative/roster fallback remains useful as a strategy's
stable default, or where no rule supplies a choice or priority. Record which
case applies. A direct instruction about timing or order takes precedence.
Simultaneous effects and nested interrupts are not interchangeable operations.

## Spending resources

The model records the outcomes required by the feature. The rules do not
establish one universal 'debit at commit, then refund on interruption' process.
`commit` and `resolution` remain internal labels for the homebrew examples;
official features use their rule's expenditure points and exceptions.

| Case | Game result to preserve |
| --- | --- |
| Ordinary weapon attack misses | The attack was made; no general refund of the action or attack allowance |
| Spell cast using a slot | Apply the normal slot expenditure and all other casting rules |
| Spell cast at an invalid target | The target is unaffected; an expended slot is still expended under the invalid-target rule |
| Target normally takes no effect after a successful save, but was invalid | Present the result as a successful save without rolling a save or revealing the invalidity |
| Revised Counterspell successfully interrupts a slot spell | The spell has no effect; its casting Action, Bonus Action or Reaction is wasted; its spell slot is not expended |
| Counterspell's target succeeds on its Constitution save | The interrupted spell is not cancelled by Counterspell; resume casting under its rules |
| Readied slot spell expires or Concentration is broken before release | The spell was cast and its resources were spent when readied; the held spell is lost without a general refund |
| Homebrew Mend or draught | Use the explicitly authored stage costs; do not infer official interruption behavior from these fixtures |

The cost ledger can use reservations internally, but reservations are not game
expenditure and must not become new player-visible timing rules. Distinguish
`not_expended` from `spent_then_recovered`: they differ for expenditure-based
limits and triggers. Only the governing rule can produce either outcome.
Never reverse unrelated costs merely because one slot was not expended.

The one-slot rule permits expenditure of only one spell slot to cast a spell
on a turn. Track that per turn, not per round or participant's own turn alone.
Slot-free casts are not slot expenditure, though other restrictions still apply.
Regaining a previously expended slot is not the same as never expending it.
Record causal expenditure facts rather than derive the rule from the current
pool balance. A Counterspell-interrupted slot that was not expended does not
count as an expended slot. This does not grant a replacement casting action.

Counterspell has its own casting requirements and costs. Its target makes a
Constitution saving throw against the counterspeller's spell save DC. It is
not the 2014 automatic interruption by comparing spell levels. Interruptions
involving slotless casting do not manufacture a recovered spell slot.
Its trigger requires seeing a creature within 60 feet casting a spell with
Verbal, Somatic or Material components. Do not offer it for an unseen casting
or assume a component-free feature necessarily triggers it. The slot exception
does not specify restoration of consumed components, item charges or every
other possible cost.

Longer casting is a different case: a spell with a casting time of one minute
or more requires the Magic action each turn and Concentration. If that
Concentration breaks, casting fails without expending a spell slot and must
restart from the beginning. It must not inherit the spent-slot behavior of a
spell that was already cast and then readied.

## Readying a spell

1. On the actor's turn, take Ready and state the perceivable trigger and chosen
   response. A readied spell must have a casting time of an action.
2. Cast that spell as normal, spending its required resources. Holding its
   energy requires Concentration even if the spell normally does not.
3. Keep the held effect only up to the start of the actor's next turn. When
   the chosen trigger finishes, the actor may spend its Reaction to release it
   or ignore that occurrence.
4. If Concentration breaks before release, the held spell dissipates without
   taking effect. Do not restore the spent slot. If the actor has no permitted
   Reaction available, it cannot release the spell by treating Ready as a free
   action.

Casting and release are distinct events. An effect that observes casting sees
the casting event; releasing held energy is not automatically a second casting
event or a second slot cost. A spell's own subsequent Concentration/duration
rules still apply once it takes effect.

Starting to cast a spell that requires Concentration ends existing Concentration
at that start point. Later interruption does not restore the old effect.
Damage, Incapacitated and death use their own Concentration rules. Readying
creates an explicit held-spell dependency; it cannot be represented solely by
delaying an ordinary action until another turn.

### Agreed scope for readied-spell targeting

Track readied spells during manual encounters, including those held by creatures.
The player or DM decides trigger interpretation and any target selection or
reselection at release. Do not force one universal target-lock timing rule.
Automatic strategies for this interpreted readied-spell behavior are outside
the current simulation scope. Do not silently choose targets or omit the action
in a trial that depends on an unresolved manual decision.

Keep an in-memory record of the actor, spell use, stated trigger, held energy,
any targets already declared, Concentration dependency, spent-resource ledger,
and expiry at the start of the actor's next turn. A target can remain undecided
until the controller supplies it. Permit the controller to release, ignore a
trigger occurrence, or cancel the held spell. Continue to track known costs,
reaction availability, Concentration loss and expiry according to the rules.
Ignoring one occurrence does not itself cancel the remaining Ready opportunity.
This adds encounter tracking, not saved run state or automatic prose execution.

## Reaction permissions and refresh

The permission belongs to the feature that grants it. Keep its source, eligible
reaction category, uses remaining, refresh event and any additional restrictions.
Do not use one `reaction_available` flag for every creature.

| Rule source | Permission | Required handling |
| --- | --- | --- |
| Core Reaction rule | Ordinary Reaction, renewed at the start of the actor's next turn | Using it prevents another ordinary Reaction until that boundary |
| Hydra, Reactive Heads | Additional reactions tied to its head count, usable only for Opportunity Attacks | Head-dependent capacity and that restriction remain explicit; they cannot pay for Shield or Protect Ally |
| Marilith, Reactive | A Reaction on every turn | Refresh at each turn boundary as its trait permits, not just its own turn |
| Legendary Actions | Separate limited uses and prescribed after-another-turn timing | Do not spend an ordinary Reaction; apply start-of-own-turn refresh, Incapacitated restriction and option-specific lockouts |

These are separate variant fixtures. Do not combine all their permissions into
one monster and invent an unsourced rule for their interaction. If a homebrew
creature intentionally combines them, its rule definition must settle that
combination. A source-specific pool can only pay an eligible use once.

Hydra has one additional reaction for each head beyond the first; its trait
does not give a different refresh time. Marilith permits one Reaction on each
turn, not unlimited Reactions within a turn.

For head-count changes, the user selected a next-turn ruling: determine Hydra's
additional reaction capacity from its current head count at the start of its
turn, then refresh those uses. Gaining or losing heads between refreshes does
not change the current reaction allowance or restore spent uses. Apply the new
count at the next start-of-turn refresh. This is an explicit local ruling for
the unclear interaction, not a claim that the published trait specifies it.
It affects reaction capacity only; other head-dependent mechanics still follow
their own rules, and death or Incapacitated still prevents reactions.

For example, four heads give three additional opportunity-attack-only reactions.
After two are spent, gaining two heads leaves one additional use until the next
turn. At that next refresh, six surviving heads give five additional uses.

## Worked checks

**Shield:** an attack total 17 hits AC 16. The target casts Shield with an
available Reaction and the required spell resource. The resulting AC 21 can
make that attack miss. The bonus remains for the spell's actual duration,
including later attacks before the start of the target's next turn. An ordinary
natural-20 critical is not defeated merely by raising AC.

**Protect Ally:** retain the already approved homebrew one-attack duration. Its
own pending-hit rule can remove a later protector's eligibility when the attack
becomes a miss. This does not establish a universal rule deleting other
abilities' already-completed triggers.

**Counterspell:** a caster with two level-one slots casts a qualifying spell;
an eligible counterspeller reacts. On the caster's failed Constitution save,
the original spell has no effect and its casting action is wasted. The caster
still has two slots. The counterspeller pays its own costs. A successful save
instead allows the original casting to continue and normally expend its slot.

**Ready:** a caster with two slots readies a level-one action spell. It has one
slot left while holding the energy. If the trigger never occurs before the
start of its next turn, the held spell is lost and it still has one slot.
If Concentration breaks first, the outcome for that slot is the same.

**Restricted reactions:** a Hydra spends an ordinary Reaction on a permitted
ordinary option and retains only those additional reactions its actual head
trait allows. Those extra uses can pay eligible Opportunity Attacks, not another
unrestricted ability. A separate Marilith fixture can react on consecutive
creatures' turns as its own trait permits. No round-based reset is substituted.

## Acceptance requirements and limits

Each operation must have a source-backed trigger, selection timing, permitted
cost outcome, state change, continuation behavior and expiry. Preserve completed
events and apply only changes the rule permits. Match the primary text before
choosing names for engine fields.

Some source-specific combinations still need a ruling; the books are not a
complete formal specification. Mark an unresolved interaction and any explicit
campaign ruling rather than silently inventing a universal ordering rule. This
does not reopen settled routine fixture choices. Targeting assumptions remain
visible, and maps/spatial simulation and runtime persistence remain deferred.
The two identified cases are now addressed: Hydra reaction capacity uses the
user's next-turn ruling; readied-spell targeting remains a manual encounter
decision outside automatic strategies. Neither needs another design question.

## Sources

Checked 2026-09-15 against official revised rules:

- [Playing the Game: Reactions](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#Reactions), also SRD 5.2.1 p10.
- [Rules Glossary](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary): Ready, Opportunity Attacks, Simultaneous Effects, Concentration, Incapacitated.
- [Spells](https://www.dndbeyond.com/sources/dnd/free-rules/spells): one spell slot per turn, targets and invalid targets, casting and components.
- [SRD 5.2.1](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf): Counterspell p120, Shield pp161–162, monster conventions p257, Hydra p300, Marilith p306.

The internal event names in the schema and the Test Warden's homebrew features
are design choices; the source citations do not claim those names occur in the
published rules.
