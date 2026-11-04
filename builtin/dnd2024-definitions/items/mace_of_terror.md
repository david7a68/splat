---
splat_schema: 2
splat_id: eca6616c-5dd6-5a04-b836-c3e227c158ae
splat_kind: item
splat_key: dnd2024:items/mace_of_terror
splat_name: Mace of Terror
splat_origin: dnd2024:reference/magic_item/mace_of_terror
splat_sources:
- SRD 5.2.1, pp. 229, https://www.dndbeyond.com/srd
---

# Mace of Terror


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 229 -->

Weapon (Mace), Rare (Requires Attunement)

This magic weapon has 3 charges and regains 1d3 expended charges daily at dawn. While holding the weapon, you can take a Magic action and expend 1 charge to release a wave of terror from it. Each creature of your choice within 30 feet of you must succeed on a DC 15 Wisdom saving throw or have the Frightened condition for 1 minute. While Frightened in this way, a creature must spend its turns trying to move as far away from you as it can, and it can’t make Opportunity Attacks. For its action, it can use only the Dash action or try to escape from an effect that prevents it from moving. If it has nowhere it can move, the creature can take the Dodge action. At the end of each of its turns, a creature repeats the save, ending the effect on itself on a success.

Original layout: [[../../srd-5.2.1/pages/229|page 229]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/mace_of_terror ` |
| ` implementation ` | ` dnd2024:item_mace_of_terror ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    expression: 1d3
    scope: use
  cap: maximum
```

#### Terror

<!-- splat:yaml path=splat_rule/parameters/terror -->

```yaml
action: magic
charge_cost: 1
targets:
  chooser: user
  range_ft: 30
save:
  ability: wisdom
  dc: 15
  failure:
    condition: frightened
    duration:
      amount: 1
      unit: minute
  success:
    effect: none
while_frightened:
  movement: as_far_away_from_user_as_possible
  opportunity_attacks: false
  actions_allowed:
  - dash
  - escape_movement_preventing_effect
  if_no_movement_possible: dodge_allowed
repeat_save:
  event: target_turn_end
  ability: wisdom
  dc: 15
  success: end_effect
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
