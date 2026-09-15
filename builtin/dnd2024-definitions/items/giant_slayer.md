---
splat_schema: 1
splat_id: 963f89d2-fadc-5614-a4b8-15170424c831
splat_kind: item
splat_key: dnd2024:items/giant_slayer
splat_name: Giant Slayer
splat_origin:
  key: dnd2024:reference/magic_item/giant_slayer
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
---

# Giant Slayer

## Source text

<!-- SRD page 223 -->

Weapon (Any Simple or Martial), Rare\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon.\
When you hit a Giant with this weapon, the Giant\
takes an extra 2d6 damage of the weapon’s type and\
must succeed on a DC 15 Strength saving throw or\
have the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/giant_slayer ` |
| ` implementation ` | ` dnd2024:item_giant_slayer ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 1 ` |
| ` damage_roll_bonus ` | ` 1 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
target_creature_type: giant
extra_damage:
  roll:
    expression: 2d6
    scope: attack
  type: weapon_damage_type
save:
  ability: strength
  dc: 15
  failure:
    condition: prone
  success:
    effect: none
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
