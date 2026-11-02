---
splat_schema: 1
splat_id: 9f2a75bc-7610-58ac-9e92-56b38e9f6025
splat_kind: item
splat_key: dnd2024:items/holy_avenger
splat_name: Holy Avenger
splat_origin:
  key: dnd2024:reference/magic_item/holy_avenger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
---

# Holy Avenger

## Source text

<!-- SRD page 226 -->

Weapon (Any Simple or Martial), Legendary (Requires Attunement by a Paladin)

You gain a +3 bonus to attack rolls and damage rolls made with this magic weapon. When you hit a Fiend or an Undead with it, that creature takes an extra 2d10 Radiant damage.

While you hold the drawn weapon, it creates a 10-foot Emanation originating from you. You and all creatures Friendly to you in the Emanation have Advantage on saving throws against spells and other magical effects. If you have 17 or more levels in the Paladin class, the size of the Emanation increases to 30 feet.

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Paladin ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/holy_avenger ` |
| ` implementation ` | ` dnd2024:item_holy_avenger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 3 ` |
| ` damage_roll_bonus ` | ` 3 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
target_types:
- fiend
- undead
damage:
  roll:
    expression: 2d10
    scope: attack
  type: radiant
```

#### Aura

<!-- splat:yaml path=splat_rule/parameters/aura -->

```yaml
equipment_state: held_drawn
shape: emanation
origin: user
radius_ft: 10
radius_override:
  paladin_level_minimum: 17
  radius_ft: 30
targets:
- self
- friendly_creatures
save_advantage_against:
- spells
- other_magical_effects
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
| ` eligible_classes ` | ` ["paladin"] ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
