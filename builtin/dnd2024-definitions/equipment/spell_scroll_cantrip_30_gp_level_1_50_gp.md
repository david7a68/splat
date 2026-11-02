---
splat_schema: 1
splat_id: 92fc1503-04b1-5c5e-b686-6f411546cc55
splat_kind: item
splat_key: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
splat_name: Spell Scroll (Cantrip, 30 GP; Level 1, 50 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Spell Scroll (Cantrip, 30 GP; Level 1, 50 GP)

## Source text

<!-- SRD page 99 -->

A Spell Scroll (Cantrip) or Spell Scroll (Level 1) is a magic item that bears the words of a cantrip or level 1 spell, respectively, determined by the scroll’s creator. If the spell is on your class’s spell list, you can read the scroll and cast the spell using its normal casting time and without providing any Material components.

If the spell requires a saving throw or an attack roll, the spell save DC is 13, and the attack bonus is +5. The scroll disintegrates when the casting is completed.

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical ` | ` true ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- cantrip
- level_1
columns:
  Variant:
    path: id
    type: symbol
  Spell level:
    path: spell_level
    type: integer
  Cost:
    path: price
    type: quantity
-->

| Variant | Spell level | Cost |
| --- | --- | --- |
| ` Cantrip ` | ` 0 ` | ` 30 GP ` |
| ` Level 1 ` | ` 1 ` | ` 50 GP ` |

#### Spell selection

<!-- splat:values path=splat_rule/parameters/spell_selection -->

| Field | Value |
| --- | --- |
| ` authority ` | ` creator ` |
| ` level ` | ` variant_spell_level ` |

#### Cast

<!-- splat:values path=splat_rule/parameters/cast -->

| Field | Value |
| --- | --- |
| ` requires_spell_on_class_list ` | ` true ` |
| ` requires_reading ` | ` true ` |
| ` casting_time ` | ` spell_normal ` |
| ` material_components_required ` | ` false ` |
| ` save_dc ` | ` 13 ` |
| ` attack_bonus ` | ` 5 ` |
| ` on_completed ` | ` scroll_disintegrates ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |
