---
splat_schema: 1
splat_id: cba41089-70d8-5b5e-98e6-a90af7674f72
splat_kind: spell
splat_key: dnd2024:spells/secret_chest
splat_name: Secret Chest
splat_origin:
  key: dnd2024:reference/spell/secret_chest
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a chest, 3 feet by 2 feet by 2 feet, constructed from rare materials worth 5,000+ GP,
      and a Tiny replica of the chest made from the same materials worth 50+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/secret_chest
  implementation: dnd2024:spell_secret_chest
  parameters:
    spell_lifetime:
      concentration: false
      until: dispelled_or_source_specific_end
    initial:
      touch_required:
      - chest
      - replica
      destination: ethereal_plane
      transported: chest_and_contents
      contents:
        nonliving: true
        maximum_cubic_feet: 12
      chest_dimensions_feet:
      - 3
      - 2
      - 2
    recall:
      economy: magic_action
      touch_required:
      - replica
      requires: chest_on_ethereal_plane
      destination:
        ground: true
        unoccupied: true
        within_caster_feet: 5
    send_back:
      economy: magic_action
      touch_required:
      - chest
      - replica
      destination: ethereal_plane
    expiration_check:
      starts_after_days: 60
      event: day.end
      probability_increment_percent_per_day: 5
      first_eligible_day_percent: 5
      maximum_percent: 100
      success: end_spell
    additional_end_conditions:
    - caster_casts_secret_chest_again
    - replica_destroyed
    end_while_chest_ethereal:
      chest_remains_ethereal: true
      recoverable_by_search: true
splat_contracts:
- id: dnd2024:spell_secret_chest
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    chest:
      type: object
      required: true
    replica:
      type: object
      required: true
    contents:
      type: object_list
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - transports
  - objects
  events:
  - day.end
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - planar_travel
  - world_objects
  - inventory
  - campaign_time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 805e87f017a62827533b487b42b995f37f7a55fc1bc285265ace1a3ef7334449
  source_path: builtin/srd-5.2.1/spell/secret_chest.md
  pages:
  - 160
  engine_gaps: &id001
  - dnd2024:spell_secret_chest
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 3983bf272fd1a5bc23d57e2c64caba6493375b2653a2b77bc83d9f1bf89b33b8
splat_clauses:
- key: source_rules
  text: '# Secret Chest Level 4 Conjuration (Wizard) Casting Time: Action Range: Touch Components: V,
    S, M (a chest, 3 feet by 2 feet by 2 feet, constructed from rare materials worth 5,000+ GP, and a
    Tiny replica of the chest made from the same materials worth 50+ GP) Duration: Until dispelled You
    hide a chest and all its contents on the Ethereal Plane. You must touch the chest and the miniature
    replica that serve as Material components for the spell. The chest can contain up to 12 cubic feet
    of nonliving material (3 feet by 2 feet by 2 feet). While the chest remains on the Ethereal Plane,
    you can take a Magic action and touch the replica to recall the chest. It appears in an unoccupied
    space on the ground within 5 feet of you. You can send the chest back to the Ethereal Plane by taking
    a Magic action to touch the chest and the replica. After 60 days, there is a cumulative 5 percent
    chance at the end of each day that the spell ends. The spell also ends if you cast this spell again
    or if the Tiny replica chest is destroyed. If the spell ends and the larger chest is on the Ethereal
    Plane, the chest remains there for you or someone else to find.'
  sha256: 2ef0564b213ef15222b5ca4d0bb86fca86a0ee79bfb714ed2a69491950668ddf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Secret Chest

<!-- SRD page 160 -->

Level 4 Conjuration (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a chest, 3 feet by 2 feet by 2 feet,\
constructed from rare materials worth 5,000+ GP, and\
a Tiny replica of the chest made from the same materi-\
als worth 50+ GP)\
Duration: Until dispelled\
You hide a chest and all its contents on the Ethereal\
Plane. You must touch the chest and the miniature\
replica that serve as Material components for the\
spell. The chest can contain up to 12 cubic feet of\
nonliving material (3 feet by 2 feet by 2 feet).\
While the chest remains on the Ethereal Plane,\
you can take a Magic action and touch the replica to\
recall the chest. It appears in an unoccupied space\
on the ground within 5 feet of you. You can send the\
chest back to the Ethereal Plane by taking a Magic\
action to touch the chest and the replica.\
After 60 days, there is a cumulative 5 percent\
chance at the end of each day that the spell ends.\
The spell also ends if you cast this spell again or if\
the Tiny replica chest is destroyed. If the spell ends\
and the larger chest is on the Ethereal Plane, the\
chest remains there for you or someone else to find.\

Original layout: [[../../srd-5.2.1/pages/160|page 160]].
