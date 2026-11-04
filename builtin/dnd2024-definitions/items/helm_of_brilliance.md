---
splat_schema: 2
splat_id: d77716f3-9bd9-5fa1-9d18-848fd77c494d
splat_kind: item
splat_key: dnd2024:items/helm_of_brilliance
splat_name: Helm of Brilliance
splat_origin: dnd2024:reference/magic_item/helm_of_brilliance
splat_sources:
- SRD 5.2.1, pp. 225, 226, https://www.dndbeyond.com/srd
---

# Helm of Brilliance


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 225 -->

Wondrous Item, Very Rare (Requires Attunement)

This helm is set with 1d10 diamonds, 2d10 rubies, 3d10 fire opals, and 4d10 opals. Any gem pried from the helm crumbles to dust. When all the gems are removed or destroyed, the helm loses its magic.

You gain the following benefits while wearing the helm.

Diamond Light. As long as it has at least one diamond, the helm emits a 30-foot Emanation. When at least one Undead is within that area, the Emanation is filled with Dim Light. Any Undead that starts its turn in that area takes 1d6 Radiant damage.

Fire Opal Flames. As long as the helm has at least one fire opal, you can take a Magic action to cause one weapon you are holding to burst into flames. The flames emit Bright Light in a 10-foot radius and Dim Light for an additional 10 feet. The flames are harmless to you and the weapon. When you hit with an attack using the blazing weapon, the target takes an extra 1d6 Fire damage. The flames last until you take a Bonus Action to extinguish them or until you drop or stow the weapon.

Ruby Resistance. As long as the helm has at least one ruby, you have Resistance to Fire damage.

Spells. You can cast one of the following spells (save DC 18), using one of the helm’s gems of the specified type as a component: Daylight (opal), Fireball (fire opal), Prismatic Spray (diamond), or Wall of Fire (ruby). The gem is destroyed when the spell is cast and disappears from the helm.

Taking Fire Damage. Roll 1d20 if you are wearing the helm and take Fire damage as a result of failing a saving throw against a spell. On a roll of 1, the helm emits beams of light from its remaining gems and is then destroyed. Each creature within a 60-foot Emanation originating from you must succeed on a DC 17 Dexterity saving throw or be struck by a beam, taking Radiant damage equal to the number of gems in the helm.

Original layout: [[../../srd-5.2.1/pages/225|page 225]], [[../../srd-5.2.1/pages/226|page 226]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/helm_of_brilliance ` |
| ` implementation ` | ` dnd2024:item_helm_of_brilliance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Gems

<!-- splat:yaml path=splat_rule/parameters/gems -->

```yaml
initial:
  diamond:
    dice: 1
    sides: 10
    bonus: 0
  ruby:
    dice: 2
    sides: 10
    bonus: 0
  fire_opal:
    dice: 3
    sides: 10
    bonus: 0
  opal:
    dice: 4
    sides: 10
    bonus: 0
pried_out: destroy_gem
none_remaining: item_loses_magic
```

#### Diamond light

<!-- splat:yaml path=splat_rule/parameters/diamond_light -->

```yaml
requires_gem: diamond
shape: emanation
radius_ft: 30
light_if_undead_present: dim
undead_turn_start_damage:
  roll:
    dice: 1
    sides: 6
    bonus: 0
  type: radiant
```

#### Fire opal flames

<!-- splat:yaml path=splat_rule/parameters/fire_opal_flames -->

```yaml
requires_gem: fire_opal
action: magic
target: one_held_weapon
light:
  bright_radius_ft: 10
  additional_dim_ft: 10
harmless_to:
- user
- weapon
on_hit_extra_damage:
  roll:
    dice: 1
    sides: 6
    bonus: 0
  type: fire
end_on:
- bonus_action_extinguish
- weapon_dropped
- weapon_stowed
```

#### Ruby resistance

<!-- splat:values path=splat_rule/parameters/ruby_resistance -->

| Field | Value |
| --- | --- |
| ` requires_gem ` | ` ruby ` |
| ` resistance ` | ` fire ` |

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- daylight
- fireball
- prismatic_spray
- wall_of_fire
columns:
  Spell:
    path: spell
    type: reference
  Save dc:
    path: save_dc
    type: integer
  Component gem:
    path: component/gem
    type: symbol
  Component quantity:
    path: component/quantity
    type: integer
  Component consumed:
    path: component/consumed
    type: boolean
  Component remove from helm:
    path: component/remove_from_helm
    type: boolean
-->

| Spell | Save dc | Component gem | Component quantity | Component consumed | Component remove from helm |
| --- | --- | --- | --- | --- | --- |
| [Daylight](<../spells/daylight.md> "dnd2024:spells/daylight") | ` 18 ` | ` Opal ` | ` 1 ` | true | true |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` 18 ` | ` Fire opal ` | ` 1 ` | true | true |
| [Prismatic Spray](<../spells/prismatic_spray.md> "dnd2024:spells/prismatic_spray") | ` 18 ` | ` Diamond ` | ` 1 ` | true | true |
| [Wall of Fire](<../spells/wall_of_fire.md> "dnd2024:spells/wall_of_fire") | ` 18 ` | ` Ruby ` | ` 1 ` | true | true |

#### Fire damage mishap

<!-- splat:yaml path=splat_rule/parameters/fire_damage_mishap -->

```yaml
trigger:
  event: fire_damage_received
  cause: failed_save_against_spell
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect:
  shape: emanation
  radius_ft: 60
  targets: all_creatures
  save:
    ability: dexterity
    dc: 17
  failure_damage:
    value_from: remaining_gem_count
    type: radiant
  success_damage: 0
  then: destroy_helm
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
