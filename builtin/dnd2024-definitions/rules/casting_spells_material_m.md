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
splat_rule:
  mechanic: dnd2024:rules/casting_spells_material_m
  parameters:
    material_source: spell_components_entry
    consumed_only_if_specified: true
    free_hand_to_access: true
    same_hand_can_perform_somatic: true
    substitution:
      allowed_if:
        consumed: false
        specified_cost: false
      options:
        component_pouch:
          free_hand_required: true
        spellcasting_focus:
          granting_feature_required: true
          held_required_unless_item_exception: true
  implementation: dnd2024:rule_casting_spells_material_m
splat_contracts:
- id: dnd2024:rule_casting_spells_material_m
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  - 106
  engine_gaps: &id001
  - dnd2024:rule_casting_spells_material_m
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: b2d10ea25184fbd2e71b09c18bded22d7e909883d826d33997b664d9a2bfaee2
  parent_name: Casting Spells
  line_span:
    first: 9212
    count: 18
splat_clauses:
- key: source_rules
  text: '# Material (M) A Material component is a particular material used in a spell’s casting, as specified
    in parentheses in the Components entry. These materials aren’t consumed by the spell unless the spell’s
    description states otherwise. The spellcaster must have a hand free to access them, but it can be
    the same hand used to perform Somatic components, if any. If a spell doesn’t consume its materials
    and doesn’t specify a cost for them, a spellcaster can use a Component Pouch (see “Equipment”) instead
    of providing the materials specified in the spell, or the spellcaster can substitute a Spellcasting
    Focus if the caster has a feature that allows that substitution. To use a Component Pouch, you must
    have a hand free to reach into it, and to use a Spellcasting Focus, you must hold it unless its description
    says otherwise (see “Equipment” for descriptions).'
  sha256: c985c1fd3a10567b9bd02995e76794981ab26d3f6e2d0a2cba25a4e362a924cc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Material (M)

<!-- SRD page 105 -->

A Material component is a particular material used\
in a spell’s casting, as specified in parentheses in\
the Components entry. These materials aren’t con-\
sumed by the spell unless the spell’s description\
states otherwise. The spellcaster must have a hand\
free to access them, but it can be the same hand\
used to perform Somatic components, if any.\
If a spell doesn’t consume its materials and\
doesn’t specify a cost for them, a spellcaster can use\
a Component Pouch (see “Equipment”) instead of\
providing the materials specified in the spell, or the\

<!-- SRD page 106 -->

spellcaster can substitute a Spellcasting Focus if the\
caster has a feature that allows that substitution. To\
use a Component Pouch, you must have a hand free\
to reach into it, and to use a Spellcasting Focus, you\
must hold it unless its description says otherwise\
(see “Equipment” for descriptions).\
