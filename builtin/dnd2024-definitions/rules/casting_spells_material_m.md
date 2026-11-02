---
splat_schema: 1
splat_id: d450f4a1-1cc0-53ea-baae-52a966ed4bff
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_material_m
splat_name: Material (M)
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105, 106
  url: https://www.dndbeyond.com/srd
---

# Material (M)

## Source text

<!-- SRD page 105 -->

A Material component is a particular material used in a spell’s casting, as specified in parentheses in the Components entry. These materials aren’t consumed by the spell unless the spell’s description states otherwise. The spellcaster must have a hand free to access them, but it can be the same hand used to perform Somatic components, if any.

If a spell doesn’t consume its materials and doesn’t specify a cost for them, a spellcaster can use a Component Pouch (see “Equipment”) instead of providing the materials specified in the spell, or the spellcaster can substitute a Spellcasting Focus if the caster has a feature that allows that substitution. To use a Component Pouch, you must have a hand free to reach into it, and to use a Spellcasting Focus, you must hold it unless its description says otherwise (see “Equipment” for descriptions).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_material_m ` |
| ` implementation ` | ` dnd2024:rule_casting_spells_material_m ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` material_source ` | ` spell_components_entry ` |
| ` consumed_only_if_specified ` | ` true ` |
| ` free_hand_to_access ` | ` true ` |
| ` same_hand_can_perform_somatic ` | ` true ` |

#### Substitution

<!-- splat:yaml path=splat_rule/parameters/substitution -->

```yaml
allowed_if:
  consumed: false
  specified_cost: false
options:
  component_pouch:
    free_hand_required: true
  spellcasting_focus:
    granting_feature_required: true
    held_required_unless_item_exception: true
```
