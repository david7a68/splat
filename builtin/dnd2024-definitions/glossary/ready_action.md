---
splat_schema: 2
splat_id: 5aae911d-50b9-50d4-a03b-6957d559bc2b
splat_kind: rule
splat_key: dnd2024:glossary/ready_action
splat_name: Ready [Action]
splat_origin: dnd2024:reference/glossary/ready_action
splat_sources:
- SRD 5.2.1, pp. 186, 187, https://www.dndbeyond.com/srd
---

# Ready [Action]

## Source text

<!-- SRD page 186 -->

You take the Ready action to wait for a particular circumstance before you act. To do so, you take this action on your turn, which lets you act by taking a Reaction before the start of your next turn.

First, you decide what perceivable circumstance will trigger your Reaction. Then, you choose the action you will take in response to that trigger, or you choose to move up to your Speed in response to it. Examples include “If the cultist steps on the trapdoor, I’ll pull the lever that opens it,” and “If the zombie steps next to me, I move away.”

<!-- SRD page 187 -->

When the trigger occurs, you can either take your Reaction right after the trigger finishes or ignore the trigger.

When you Ready a spell, you cast it as normal (expending any resources used to cast it) but hold its energy, which you release with your Reaction when the trigger occurs. To be readied, a spell must have a casting time of an action, and holding on to the spell’s magic requires Concentration, which you can maintain up to the start of your next turn. If your Concentration is broken, the spell dissipates without taking effect.

Original layout: [[../../srd-5.2.1/pages/186|page 186]], [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/ready_action ` |
| ` implementation ` | ` dnd2024:glossary_ready_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` timing ` | ` own_turn ` |
| ` expires ` | ` own_next_turn_start ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` must_be_perceivable ` | ` true ` |

#### Response

<!-- splat:values path=splat_rule/parameters/response -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` options ` | ` ["chosen_action", "move_up_to_speed"] ` |
| ` cost ` | ` reaction ` |
| ` timing ` | ` immediately_after_trigger_finishes ` |
| ` may_ignore_trigger ` | ` true ` |

#### Spell

<!-- splat:yaml path=splat_rule/parameters/spell -->

```yaml
casting_time_required: action
cast_when_readied: true
casting_resources_expended_when_readied: true
energy_held_until_release: true
release_cost: reaction
concentration_required: true
maximum_hold_until: own_next_turn_start
concentration_loss:
  dissipates: true
  effect_occurs: false
```
