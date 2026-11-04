---
splat_schema: 2
splat_id: 2d7b19f1-002c-505c-a275-8ca535664a40
splat_kind: item
splat_key: dnd2024:equipment/arcane_focus_varies
splat_name: Arcane Focus (Varies)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Arcane Focus (Varies)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

An Arcane Focus takes one of the forms in the Arcane Focuses table and is bejeweled or carved to channel arcane magic. A Sorcerer, Warlock, or Wizard can use such an item as a Spellcasting Focus.

**Arcane Focuses**

| Focus | Weight | Cost |
| --- | --- | --- |
| Crystal | 1 lb. | 10 GP |
| Orb | 3 lb. | 20 GP |
| Rod | 2 lb. | 10 GP |
| Staff (also a Quarterstaff) | 4 lb. | 5 GP |
| Wand | 1 lb. | 10 GP |





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/arcane_focus_varies ` |
| ` implementation ` | ` dnd2024:equipment_arcane_focus_varies ` |

### Parameters

#### Spellcasting focus

<!-- splat:yaml path=splat_rule/parameters/spellcasting_focus -->

```yaml
eligible_classes:
- key: dnd2024:classes/sorcerer
- key: dnd2024:classes/warlock
- key: dnd2024:classes/wizard
rule:
  key: dnd2024:glossary/spellcasting_focus
```

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- crystal
- orb
- rod
- staff
- wand
columns:
  Variant:
    path: id
    type: symbol
  Weight:
    path: mass
    type: quantity
  Cost:
    path: price
    type: quantity
  Valid use states:
    path: valid_use_states
    type: yaml
  Also weapon:
    path: also_weapon
    type: reference
    optional: true
-->

| Variant | Weight | Cost | Valid use states | Also weapon |
| --- | --- | --- | --- | --- |
| ` Crystal ` | ` 1 lb ` | ` 10 GP ` | ` ["held"] ` |  |
| ` Orb ` | ` 3 lb ` | ` 20 GP ` | ` ["held"] ` |  |
| ` Rod ` | ` 2 lb ` | ` 10 GP ` | ` ["held"] ` |  |
| ` Staff ` | ` 4 lb ` | ` 5 GP ` | ` ["held"] ` | [Quarterstaff](<quarterstaff.md> "dnd2024:equipment/quarterstaff") |
| ` Wand ` | ` 1 lb ` | ` 10 GP ` | ` ["held"] ` |  |
