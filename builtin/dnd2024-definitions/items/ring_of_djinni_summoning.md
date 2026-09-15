---
splat_schema: 1
splat_id: e25bf248-a0a4-5827-99b4-6f30e12406ec
splat_kind: item
splat_key: dnd2024:items/ring_of_djinni_summoning
splat_name: Ring of Djinni Summoning
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_djinni_summoning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 237
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_djinni_summoning
  implementation: dnd2024:item_ring_of_djinni_summoning
  parameters:
    equipped: worn
    action: magic
    summon:
      creature:
        key: dnd2024:creatures/djinni
      identity: same_particular_djinni
      origin_plane: elemental_air
      space:
        unoccupied: true
        choice: user
        range_ft: 120
      duration:
        amount: 1
        unit: hours
      concentration: true
      ends_on:
      - concentration_ends
      - zero_hp
      attitude: friendly_to_user_and_allies
      obeys_commands: true
      no_command_behavior: defend_self_otherwise_no_actions
    cooldown:
      from: djinni_departure
      duration:
        amount: 24
        unit: hours
    djinni_dies:
      item_magic: false
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_djinni_summoning
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.activation_requested
  - concentration.ended
  - hp.changed
  - participant.died
  - time.elapsed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - effect_duration
  - item_recharge
  - spatial_targeting
  - summoning
  - planar_travel
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7086fecb92399e97ec3571dfe53c0d0343d1d4117256dd4e4aa5f3e22c7ebbc7
  source_path: builtin/srd-5.2.1/magic_item/ring_of_djinni_summoning.md
  pages:
  - 237
  engine_gaps: &id001
  - dnd2024:item_ring_of_djinni_summoning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: d553b78265ba4dd15d38577bbaa777ed5cfe85318d0f26b0d0b9e742c46a8a55
splat_clauses:
- key: source_rules
  text: '# Ring of Djinni Summoning Ring, Legendary (Requires Attunement) While wearing this ring, you
    can take a Magic action to summon a particular Djinni from the Elemental Plane of Air. The djinni
    appears in an unoccupied space you choose within 120 feet of yourself. It remains as long as you maintain
    Concentration, to a maximum of 1 hour, or until it drops to 0 Hit Points. While summoned, the djinni
    is Friendly to you and your allies, and it obeys your commands. If you fail to command it, the djinni
    defends itself against attackers but takes no other actions. After the djinni departs, it can’t be
    summoned again for 24 hours, and the ring becomes nonmagical if the djinni dies. Rings of Djinni Summoning
    are often created by the djinn they summon and given to mortals as gifts of friendship or tokens of
    esteem.'
  sha256: 6c4dcb09a0d7d71d2bc882119b56b2620c9dfc2186a189523a005125d18376d8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Djinni Summoning

<!-- SRD page 237 -->

Ring, Legendary (Requires Attunement)\
While wearing this ring, you can take a Magic action\
to summon a particular Djinni from the Elemental\
Plane of Air. The djinni appears in an unoccupied\
space you choose within 120 feet of yourself. It\
remains as long as you maintain Concentration, to a\
maximum of 1 hour, or until it drops to 0 Hit Points.\
While summoned, the djinni is Friendly to you and\
your allies, and it obeys your commands. If you fail\
to command it, the djinni defends itself against at-\
tackers but takes no other actions.\
After the djinni departs, it can’t be summoned\
again for 24 hours, and the ring becomes nonmagi-\
cal if the djinni dies.\
Rings of Djinni Summoning are often created by the\
djinn they summon and given to mortals as gifts of\
friendship or tokens of esteem.\

Original layout: [[../../srd-5.2.1/pages/237|page 237]].
