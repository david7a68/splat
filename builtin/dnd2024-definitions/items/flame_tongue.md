---
splat_schema: 2
splat_id: 03f997cb-7c4e-58c3-997a-c948ba0ef9ab
splat_kind: item
splat_key: dnd2024:items/flame_tongue
splat_name: Flame Tongue
splat_origin: dnd2024:reference/magic_item/flame_tongue
splat_sources:
- SRD 5.2.1, pp. 223, https://www.dndbeyond.com/srd
---

# Flame Tongue


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 223 -->

Weapon (Any Melee Weapon), Rare (Requires Attunement)

While holding this magic weapon, you can take a Bonus Action and use a command word to cause flames to engulf the damage-dealing part of the weapon. These flames shed Bright Light in a 40-foot radius and Dim Light for an additional 40 feet. While the weapon is ablaze, it deals an extra 2d6 Fire damage on a hit. The flames last until you take a Bonus Action to issue the command again or until you drop, stow, or sheathe the weapon.

Original layout: [[../../srd-5.2.1/pages/223|page 223]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/flame_tongue ` |
| ` implementation ` | ` dnd2024:item_flame_tongue ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |
| ` ends_on ` | ` ["drop", "stow", "sheathe"] ` |

#### Ignite

<!-- splat:values path=splat_rule/parameters/ignite -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` command_word ` | ` true ` |

#### Ablaze

<!-- splat:yaml path=splat_rule/parameters/ablaze -->

```yaml
bright_radius_ft: 40
dim_additional_ft: 40
on_hit:
  damage:
    roll:
      expression: 2d6
      scope: attack
    type: fire
```

#### Extinguish

<!-- splat:values path=splat_rule/parameters/extinguish -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` command_word ` | ` true ` |

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
