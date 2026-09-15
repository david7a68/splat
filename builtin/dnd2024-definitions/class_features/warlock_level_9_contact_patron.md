---
splat_schema: 1
splat_id: a95fdbb4-88c5-5f1d-894d-e5f38f436ae0
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_9_contact_patron
splat_name: 'Level 9: Contact Patron'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_9_contact_patron
  parameters:
    always_prepared:
    - key: dnd2024:spells/contact_other_plane
    counts_against_prepared_limit: false
    free_cast:
      resource:
        id: warlock.contact_patron
        maximum: 1
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      contact: actor.patron
      expend_slot: false
      spell_saving_throw: automatic_success
    class_scope:
      class: warlock
      minimum_level: 9
  implementation: dnd2024:class_features_warlock_level_9_contact_patron
splat_contracts:
- id: dnd2024:class_features_warlock_level_9_contact_patron
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_9_contact_patron
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0580fbee53d06ee63843877a24f3fc076993a06b82b4761952d0a59a8f2142d0
  parent_name: Warlock
  line_span:
    first: 6108
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 9: Contact Patron In the past, you usually contacted your patron through intermediaries.
    Now you can communicate directly; you always have the Contact Other Plane spell prepared. With this
    feature, you can cast the spell without expending a spell slot to contact your patron, and you automatically
    succeed on the spell’s saving throw. Once you cast the spell with this feature, you can’t do so in
    this way again until you finish a Long Rest.'
  sha256: ccfa1f865281d7c496e861986fa1dcc3556e0fda85d526df9a10fb104a1d010f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Contact Patron

<!-- SRD page 72 -->

In the past, you usually contacted your patron\
through intermediaries. Now you can communicate\
directly; you always have the Contact Other Plane\
spell prepared. With this feature, you can cast the\
spell without expending a spell slot to contact your\
patron, and you automatically succeed on the spell’s\
saving throw.\
Once you cast the spell with this feature, you can’t\
do so in this way again until you finish a Long Rest.\
