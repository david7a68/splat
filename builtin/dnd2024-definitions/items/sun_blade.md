---
splat_schema: 2
splat_id: 5afb35f3-dc56-5f6d-8f4c-a92cecbc4719
splat_kind: item
splat_key: dnd2024:items/sun_blade
splat_name: Sun Blade
splat_origin: dnd2024:reference/magic_item/sun_blade
splat_sources:
- SRD 5.2.1, pp. 248, https://www.dndbeyond.com/srd
---

# Sun Blade


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 248 -->

Weapon (Longsword), Rare (Requires Attunement)

This item appears to be a sword hilt.

Blade of Radiance. While grasping the hilt, you can take a Bonus Action to cause a blade of pure radiance to spring into existence or make the blade disappear. While the blade exists, this magic weapon functions as a Longsword with the Finesse property. If you are proficient with Longswords or Shortswords, you are proficient with the Sun Blade.

You gain a +2 bonus to attack rolls and damage rolls made with this weapon, which deals Radiant damage instead of Slashing damage. When you hit an Undead with it, that target takes an extra 1d8 Radiant damage.

Sunlight. The sword’s luminous blade emits Bright Light in a 15-foot radius and Dim Light for an additional 15 feet. The light is sunlight. While the blade persists, you can take a Magic action to expand or reduce its radius of Bright Light and Dim Light by 5 feet each, to a maximum of 30 feet each or a minimum of 10 feet each.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sun_blade ` |
| ` implementation ` | ` dnd2024:item_sun_blade ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` default_form ` | ` hilt ` |
| ` attunement_required ` | ` true ` |

#### Blade toggle

<!-- splat:values path=splat_rule/parameters/blade_toggle -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` requires ` | ` grasping_hilt ` |

#### While blade exists

<!-- splat:yaml path=splat_rule/parameters/while_blade_exists -->

```yaml
weapon:
  key: dnd2024:equipment/longsword
properties_add:
- finesse
proficient_if_any:
- longsword_proficiency
- shortsword_proficiency
attack_bonus: 2
damage_bonus: 2
replace_damage_type:
  from: slashing
  to: radiant
on_hit:
  target_type: undead
  damage:
    amount:
      roll:
        count: 1
        sides: 8
    type: radiant
light:
  bright_radius_ft: 15
  additional_dim_ft: 15
  sunlight: true
adjust_light:
  action: magic
  increment_ft: 5
  minimum_bright_and_additional_dim_ft: 10
  maximum_bright_and_additional_dim_ft: 30
  adjust_both_together: true
```
