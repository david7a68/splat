---
splat_schema: 1
splat_id: 63bbcc54-2d6c-5efc-8b0b-1c27e0ce900a
splat_kind: item
splat_key: dnd2024:items/ring_of_mind_shielding
splat_name: Ring of Mind Shielding
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_mind_shielding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_mind_shielding
  implementation: dnd2024:item_ring_of_mind_shielding
  parameters:
    equipment_state: worn
    magic_immunities:
    - read_thoughts
    - detect_lie
    - know_alignment
    - know_creature_type
    incoming_telepathy:
      requires_consent: true
      exception: resident_ring_soul
    imperceptibility:
      enable_action: magic
      end_on:
      - magic_action_make_perceptible
      - ring_removed
      - wearer_death
    soul:
      trigger: wearer_dies
      requires: ring_has_no_soul
      effect: soul_enters_ring
      choice:
      - remain
      - depart_afterlife
      resident_telepathy:
        target: current_wearer
        wearer_cannot_block: true
      capacity: 1
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_ring_of_mind_shielding
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    campaign:
      required: false
      type: campaign
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8bc016a2a4d7dbbff990bd72e0d969b1a09e79292a708aa94666392e782f1a5c
  source_path: builtin/srd-5.2.1/magic_item/ring_of_mind_shielding.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_mind_shielding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 0b980d40113d9cc15a3b62bd142bdccf17ca2f21109d4c1018232cafae3bcb9c
splat_clauses:
- key: source_rules
  text: '# Ring of Mind Shielding Ring, Uncommon (Requires Attunement) While wearing this ring, you are
    immune to magic that allows other creatures to read your thoughts, determine whether you are lying,
    know your alignment, or know your creature type. Creatures can telepathically communicate with you
    only if you allow it. You can take a Magic action to cause the ring to become imperceptible until
    you take another Magic action to make it perceptible, until you remove the ring, or until you die.
    If you die while wearing the ring, your soul enters it, unless it already houses a soul. You can remain
    in the ring or depart for the afterlife. As long as your soul is in the ring, you can telepathically
    communicate with any creature wearing it. A wearer can’t prevent this telepathic communication.'
  sha256: 94a09280a06b5fe909cd8ba50aa8a8a6a5aa770eea8d7725240bd0d2a55cefea
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Mind Shielding

<!-- SRD page 238 -->

Ring, Uncommon (Requires Attunement)\
While wearing this ring, you are immune to magic\
that allows other creatures to read your thoughts,\
determine whether you are lying, know your align-\
ment, or know your creature type. Creatures can\
telepathically communicate with you only if you\
allow it.\
You can take a Magic action to cause the ring to\
become imperceptible until you take another Magic\
action to make it perceptible, until you remove the\
ring, or until you die.\
If you die while wearing the ring, your soul enters\
it, unless it already houses a soul. You can remain in\
the ring or depart for the afterlife. As long as your\
soul is in the ring, you can telepathically commu-\
nicate with any creature wearing it. A wearer can’t\
prevent this telepathic communication.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].
