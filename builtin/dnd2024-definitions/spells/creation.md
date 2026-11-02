---
splat_schema: 1
splat_id: d41bb77c-c62e-5e75-ad2b-d04aff455eb0
splat_kind: spell
splat_key: dnd2024:spells/creation
splat_name: Creation
splat_origin:
  key: dnd2024:reference/spell/creation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 121
  url: https://www.dndbeyond.com/srd
---

# Creation

## Source text

<!-- SRD page 121 -->

Level 5 Illusion (Sorcerer, Wizard)\
Casting Time: 1 minute\
Range: 30 feet\
Components: V, S, M (a paintbrush)\
Duration: Special

You pull wisps of shadow material from the Shadowfell to create an object within range. It is either an object of vegetable matter (soft goods, rope, wood, and the like) or mineral matter (stone, crystal, metal, and the like). The object must be no larger than a 5-foot Cube, and the object must be of a form and material that you have seen.

The spell’s duration depends on the object’s material, as shown in the Materials table. If the object is composed of multiple materials, use the shortest duration. Using any object created by this spell as another spell’s Material component causes the other spell to fail.

**Materials**

| Material | Duration |
| --- | --- |
| Vegetable matter | 24 hours |
| Stone or crystal | 12 hours |
| Precious metals | 1 hour |
| Gems | 10 minutes |
| Adamantine or mithral | 1 minute |

Using a Higher-Level Spell Slot. The Cube increases by 5 feet for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/121|page 121]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Special ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a paintbrush ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/creation ` |
| ` implementation ` | ` dnd2024:spell_creation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` mixed_material_duration ` | ` minimum_component_duration ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` determined_by ` | ` material_duration_table ` |

#### Object

<!-- splat:yaml path=splat_rule/parameters/object -->

```yaml
count: 1
range_feet: 30
permitted_matter:
- vegetable
- mineral
form_seen_before: true
material_seen_before: true
maximum_cube_side_feet:
  base: 5
  increment_per_slot: 5
  base_slot: 5
```

#### Duration by material seconds

<!-- splat:values path=splat_rule/parameters/duration_by_material_seconds -->

| Field | Value |
| --- | --- |
| ` vegetable ` | ` 86400 ` |
| ` stone ` | ` 43200 ` |
| ` crystal ` | ` 43200 ` |
| ` precious_metal ` | ` 3600 ` |
| ` gems ` | ` 600 ` |
| ` adamantine ` | ` 60 ` |
| ` mithral ` | ` 60 ` |

#### Unlisted mineral duration

<!-- splat:values path=splat_rule/parameters/unlisted_mineral_duration -->

| Field | Value |
| --- | --- |
| ` kind ` | ` unresolved_source_case ` |
| ` required_input ` | ` material_duration_resolution ` |
| ` no_automatic_default ` | ` true ` |

#### Used as other spell material

<!-- splat:values path=splat_rule/parameters/used_as_other_spell_material -->

| Field | Value |
| --- | --- |
| ` effect ` | ` other_spell_fails ` |
