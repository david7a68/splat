---
splat_schema: 2
splat_id: d1142589-53f6-57af-acbc-8959df188421
splat_kind: item
splat_key: dnd2024:items/staff_of_thunder_and_lightning
splat_name: Staff of Thunder and Lightning
splat_origin: dnd2024:reference/magic_item/staff_of_thunder_and_lightning
splat_sources:
- SRD 5.2.1, pp. 247, 248, https://www.dndbeyond.com/srd
---

# Staff of Thunder and Lightning


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 247 -->

Staff, Very Rare (Requires Attunement)

This staff can be wielded as a magic Quarterstaff that grants a +2 bonus to attack rolls and damage rolls made with it. It also has the following additional properties. Once one of these properties is used, it can’t be used again until the next dawn.

Lightning. When you hit with a melee attack using the staff, you can cause the target to take an extra 2d6 Lightning damage (no action required).

Thunder. When you hit with a melee attack using the staff, you can cause the staff to emit a crack of thunder audible out to 300 feet (no action required). The target you hit must succeed on a DC 17 Constitution saving throw or have the Stunned condition until the end of your next turn.

<!-- SRD page 248 -->

Thunder and Lightning. Immediately after you hit with a melee attack using the staff, you can take a Bonus Action to use the Lightning and Thunder properties (see above) at the same time. Doing so doesn’t expend the daily use of those properties, only the use of this one.

Lightning Strike. You can take a Magic action to cause a bolt of lightning to leap from the staff’s tip in a Line that is 5 feet wide and 120 feet long. Each creature in that Line makes a DC 17 Dexterity saving throw, taking 9d6 Lightning damage on a failed save or half as much damage on a successful one.

Thunderclap. You can take a Magic action to cause the staff to produce a thunderclap audible out to 600 feet. Every creature within a 60-foot Emanation originating from you makes a DC 17 Constitution saving throw. On a failed save, a creature takes 2d6 Thunder damage and has the Deafened condition for 1 minute. On a successful save, a creature takes half as much damage only.

Original layout: [[../../srd-5.2.1/pages/247|page 247]], [[../../srd-5.2.1/pages/248|page 248]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_thunder_and_lightning ` |
| ` implementation ` | ` dnd2024:item_staff_of_thunder_and_lightning ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 2 ` |
| ` damage_roll_bonus ` | ` 2 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### Weapon

<!-- splat:values path=splat_rule/parameters/weapon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quarterstaff ` |

#### Properties each independent uses

<!-- splat:yaml path=splat_rule/parameters/properties_each_independent_uses -->

```yaml
maximum: 1
initial: 1
recovery:
  event: dawn
  amount:
    expression: '1'
    scope: use
  cap: maximum
```

#### Lightning

<!-- splat:yaml path=splat_rule/parameters/lightning -->

```yaml
trigger: melee_hit_with_staff
action: none
damage:
  roll:
    expression: 2d6
    scope: attack
  type: lightning
```

#### Thunder

<!-- splat:yaml path=splat_rule/parameters/thunder -->

```yaml
trigger: melee_hit_with_staff
action: none
audible_ft: 300
save:
  ability: constitution
  dc: 17
  failure:
    condition: stunned
    duration:
      until: end_of_user_next_turn
  success:
    effect: none
```

#### Thunder and lightning

<!-- splat:values path=splat_rule/parameters/thunder_and_lightning -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` immediately_after_melee_hit_with_staff ` |
| ` action ` | ` bonus_action ` |
| ` apply_properties ` | ` ["lightning", "thunder"] ` |
| ` consume_daily_uses ` | ` ["thunder_and_lightning"] ` |
| ` consume_other_daily_uses ` | ` false ` |

#### Lightning strike

<!-- splat:yaml path=splat_rule/parameters/lightning_strike -->

```yaml
action: magic
area:
  shape: line
  width_ft: 5
  length_ft: 120
save:
  ability: dexterity
  dc: 17
  failure:
    damage:
      roll:
        expression: 9d6
        scope: use
      type: lightning
  success:
    damage_scale:
      numerator: 1
      denominator: 2
```

#### Thunderclap

<!-- splat:yaml path=splat_rule/parameters/thunderclap -->

```yaml
action: magic
audible_ft: 600
area:
  shape: emanation
  radius_ft: 60
  origin: user
save:
  ability: constitution
  dc: 17
  failure:
    damage:
      roll:
        expression: 2d6
        scope: use
      type: thunder
    condition: deafened
    duration:
      amount: 1
      unit: minute
  success:
    damage_scale:
      numerator: 1
      denominator: 2
    condition: null
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
| ` 0 ` | ` dnd2024:rules/magic_item_categories_staffs ` |
