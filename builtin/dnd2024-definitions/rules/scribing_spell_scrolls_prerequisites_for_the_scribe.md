---
splat_schema: 1
splat_id: a095ba0d-f74e-5f96-8cf6-b48a1f75092a
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe
splat_name: Prerequisites for the Scribe
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe
  parameters:
    proficiency_any:
    - arcana
    - calligraphers_supplies
    spell_prepared_each_work_day: true
    material_components_available: true
    consumed_components_consumption_timing: scroll_completion
    scroll_save_dc: scribe_spell_save_dc
    scroll_attack_bonus: scribe_spell_attack_bonus
  implementation: dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe
splat_contracts:
- id: dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - crafted_items
  events: []
  capabilities:
  - crafting
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: b2a70ffc5f92f6c9bfa2a7189fa15b6571b5710e927700cc1aa41c377d558be5
  parent_name: Scribing Spell Scrolls
  line_span:
    first: 9005
    count: 9
splat_clauses:
- key: source_rules
  text: '# Prerequisites for the Scribe To scribe a scroll, you must have proficiency in the Arcana skill
    or with Calligrapher’s Supplies and have the spell prepared on each day of the inscription. You must
    also have at hand any Material components required by the spell; if the spell consumes its Material
    components, they are consumed only when you complete the scroll. The scroll’s spell uses your spell
    save DC and spell attack bonus.'
  sha256: 2cd61707d0575f3d8d535e86560d7945fc74a6910190900fd324240a5a405ba9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prerequisites for the Scribe

<!-- SRD page 103 -->

To scribe a scroll, you must have proficiency in the\
Arcana skill or with Calligrapher’s Supplies and\
have the spell prepared on each day of the inscrip-\
tion. You must also have at hand any Material com-\
ponents required by the spell; if the spell consumes\
its Material components, they are consumed only\
when you complete the scroll. The scroll’s spell uses\
your spell save DC and spell attack bonus.\
