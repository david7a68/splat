---
splat_schema: 1
splat_id: 6770be22-3278-5613-aa2c-eeeb4cd26a97
splat_kind: rule
splat_key: dnd2024:glossary/spellcasting_focus
splat_name: Spellcasting Focus
splat_origin:
  key: dnd2024:reference/glossary/spellcasting_focus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/spellcasting_focus
  parameters:
    replaces: material_components
    eligibility:
      creature_can_use_focus_type: true
      material_consumed: false
      material_cost_specified: false
    class_focus_permissions: class_description_defined
  implementation: dnd2024:glossary_spellcasting_focus
splat_contracts:
- id: dnd2024:glossary_spellcasting_focus
  inputs:
    caster:
      type: participant
      required: true
    focus:
      type: item_instance
      required: true
    spell:
      type: definition
      required: true
  outputs:
  - material_component_substitution
  events:
  - spell.cast_pending
  capabilities:
  - spellcasting
  - equipment
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d22fc1ad5bf34097ecd8a73bcd7b396f10f320fb0098e64429c88b28fa6eb688
  source_path: builtin/srd-5.2.1/glossary/spellcasting_focus.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_spellcasting_focus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 86b15c860062daa3ed3b1e6349ccc95fe5ecff69f83271dc01757057a90d90fa
splat_clauses:
- key: source_rules
  text: '# Spellcasting Focus A Spellcasting Focus is an object that certain creatures can use in place
    of a spell’s Material components if those materials aren’t consumed by the spell and don’t have a
    cost specified. Some classes allow its members to use certain types of Spellcasting Focuses. See also
    “Spells” (“Casting Spells”).'
  sha256: 5686edd06a17bd08c935cb5383e83014420525c0a76fcd2080fbd5837fab6bba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spellcasting Focus

<!-- SRD page 188 -->

A Spellcasting Focus is an object that certain crea-\
tures can use in place of a spell’s Material compo-\
nents if those materials aren’t consumed by the\
spell and don’t have a cost specified. Some classes\
allow its members to use certain types of Spellcast-\
ing Focuses. See also “Spells” (“Casting Spells”).\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].
