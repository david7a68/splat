---
splat_schema: 1
splat_id: b22461fd-0a88-51ea-9cd9-05f24fdce28a
splat_kind: rule
splat_key: dnd2024:rules/warlock_investment_of_the_chain_master
splat_name: Investment of the Chain Master
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Investment of the Chain Master

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock, Pact of the Chain Invocation

When you cast Find Familiar, you infuse the summoned familiar with a measure of your eldritch power, granting the creature the following benefits.

Aerial or Aquatic. The familiar gains either a Fly Speed or a Swim Speed (your choice) of 40 feet.

Quick Attack. As a Bonus Action, you can command the familiar to take the Attack action.

Necrotic or Radiant Damage. Whenever the familiar deals Bludgeoning, Piercing, or Slashing damage, you can make it deal Necrotic or Radiant damage instead.

Your Save DC. If the familiar forces a creature to make a saving throw, it uses your spell save DC.

Resistance. When the familiar takes damage, you can take a Reaction to grant it Resistance against that damage.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_investment_of_the_chain_master ` |
| ` implementation ` | ` dnd2024:rules_warlock_investment_of_the_chain_master ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` find_familiar.cast ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 5 ` |
| ` invocation ` | ` pact_of_the_chain ` |

#### Familiar benefits

<!-- splat:yaml path=splat_rule/parameters/familiar_benefits -->

```yaml
movement_choice:
  options:
  - fly
  - swim
  speed_feet: 40
command_attack:
  economy: bonus_action
  familiar_action: attack
damage_type_choice:
  replace_types:
  - bludgeoning
  - piercing
  - slashing
  options:
  - necrotic
  - radiant
  optional: true
  timing: each_damage_instance
saving_throw_dc: caster_spell_save_dc
damage_resistance:
  trigger: familiar.damage_pending
  economy: reaction
  scope: triggering_damage
```
