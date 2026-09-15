---
splat_schema: 1
splat_id: 6965e670-1196-540c-89e2-18dc4b69b893
splat_kind: spell
splat_key: dnd2024:spells/teleportation_circle
splat_name: Teleportation Circle
splat_origin:
  key: dnd2024:reference/spell/teleportation_circle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 168, 169
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: conjuration
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - material
  concentration: false
  duration: 1 round
  casting_time: 1 minute
  ritual: false
  range:
    feet: 10
  material:
    description: rare inks worth 50+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/teleportation_circle
  implementation: dnd2024:spell_teleportation_circle
  parameters:
    spell_lifetime:
      concentration: false
      maximum_rounds: 1
    origin:
      ground: true
      range_feet: 10
      circle_radius_feet: 5
      draw_sigils_during_cast: true
    destination:
      permanent_teleportation_circle: true
      sigil_sequence_known: true
      same_plane: true
      chooser: caster
    portal:
      until:
        event: turn.end
        subject: caster
        occurrence: next
      trigger: creature.entered_portal
      transport: instantaneous
      arrival_within_destination_feet: 5
      occupied_fallback: nearest_unoccupied_space
    knowledge:
      on_first_gain_casting_ability:
        learn_sequences: 2
        destination_plane: material
        chooser: gm
      additional_sequence_study_seconds: 60
      sigil_sequences: unique_per_circle
    permanence:
      same_location: true
      frequency: daily
      cast_days: 365
      result: permanent_teleportation_circle
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_teleportation_circle
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    destination:
      type: location
      required: true
    campaign:
      type: campaign
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - transports
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - planar_travel
  - areas
  - memory
  - campaign_time
splat_conversion:
  status: specified
  category: spell
  source_sha256: a8dc71e577c8a0820b07ac9f5fb0d0fe046b9686868912dd6e99e898c4105e84
  source_path: builtin/srd-5.2.1/spell/teleportation_circle.md
  pages:
  - 168
  - 169
  engine_gaps: &id001
  - dnd2024:spell_teleportation_circle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 8e5a87907db545ada4a96a55bc4ec0cbc74baf7f4ae8580228c7635bdba8e3f5
splat_clauses:
- key: source_rules
  text: '# Teleportation Circle Level 5 Conjuration (Bard, Sorcerer, Warlock, Wizard) Casting Time: 1
    minute Range: 10 feet Components: V, M (rare inks worth 50+ GP, which the spell consumes) Duration:
    1 round As you cast the spell, you draw a 5-foot-radius circle on the ground inscribed with sigils
    that link your location to a permanent teleportation circle of your choice whose sigil sequence you
    know and that is on the same plane of existence as you. A shimmering portal opens within the circle
    you drew and remains open until the end of your next turn. Any creature that enters the portal instantly
    appears within 5 feet of the destination circle or in the nearest unoccupied space if that space is
    occupied. Many major temples, guildhalls, and other important places have permanent teleportation
    circles. Each circle includes a unique sigil sequence—a string of runes arranged in a particular pattern.
    When you first gain the ability to cast this spell, you learn the sigil sequences for two destinations
    on the Material Plane, determined by the GM. You might learn additional sigil sequences during your
    adventures. You can commit a new sigil sequence to memory after studying it for 1 minute. You can
    create a permanent teleportation circle by casting this spell in the same location every day for 365
    days.'
  sha256: bb94dafc067022376904d6e87f2941d5dd3ffb2ba98b5afbff33d1d713d97e4f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Teleportation Circle

<!-- SRD page 168 -->

Level 5 Conjuration (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: 1 minute\
Range: 10 feet\
Components: V, M (rare inks worth 50+ GP, which the\
spell consumes)\
Duration: 1 round\
As you cast the spell, you draw a 5-foot-radius circle\
on the ground inscribed with sigils that link your\
location to a permanent teleportation circle of your\
choice whose sigil sequence you know and that is\
on the same plane of existence as you. A shimmer-\
ing portal opens within the circle you drew and\
remains open until the end of your next turn. Any\
creature that enters the portal instantly appears\
within 5 feet of the destination circle or in the near-\
est unoccupied space if that space is occupied.\

<!-- SRD page 169 -->

Many major temples, guildhalls, and other import-\
ant places have permanent teleportation circles.\
Each circle includes a unique sigil sequence—a\
string of runes arranged in a particular pattern.\
When you first gain the ability to cast this spell,\
you learn the sigil sequences for two destinations\
on the Material Plane, determined by the GM. You\
might learn additional sigil sequences during your\
adventures. You can commit a new sigil sequence to\
memory after studying it for 1 minute.\
You can create a permanent teleportation circle\
by casting this spell in the same location every day\
for 365 days.\

Original layout: [[../../srd-5.2.1/pages/168|page 168]], [[../../srd-5.2.1/pages/169|page 169]].
