---
splat_schema: 1
splat_id: dd34418b-5021-57cc-b928-0e9d53f175d4
splat_kind: spell
splat_key: dnd2024:spells/gate
splat_name: Gate
splat_origin:
  key: dnd2024:reference/spell/gate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 135
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: conjuration
  classes:
  - cleric
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a diamond worth 5,000+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/gate
  implementation: dnd2024:spell_gate
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    portal:
      origin:
        unoccupied: true
        visible: true
        range_feet: 60
      destination:
        precise_location: true
        different_plane: true
      shape: circle
      diameter_feet:
        minimum: 5
        maximum: 20
        chooser: caster
      orientation:
        chooser: caster
      destination_visible_through_portal: true
      faces:
      - front
      - back
    traversal:
      entry_face: front_only
      entity: anything
      timing: instantaneous
      destination: nearest_unoccupied_space_to_other_portal
    planar_ruler_veto:
      actors:
      - deity
      - planar_ruler
      locations:
      - in_presence
      - within_domain
      choice: prevent_portal_opening
    named_creature_option:
      timing: cast
      name_kind: actual_name
      forbidden_name_kinds:
      - pseudonym
      - title
      - nickname
      requires: target_on_different_plane
      portal_destination: next_to_named_creature
      forced_transport: nearest_unoccupied_space_on_caster_side
      control_granted: false
      subsequent_behavior:
        chooser: gm
        actor: summoned_creature
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_gate
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
    orientation:
      type: choice
      required: true
    diameter_feet:
      type: number
      required: true
    creature_name:
      required: false
      type: text
    ruler_veto:
      required: false
      type: choice
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - transports
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - planar_travel
  - areas
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7efadd5782a7a53ae87cac37a2b9717a3781a559ef8598c625c47253f6b10f7c
  source_path: builtin/srd-5.2.1/spell/gate.md
  pages:
  - 135
  engine_gaps: &id001
  - dnd2024:spell_gate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: d494dfbb2a4a65d9d1f371f14b9f6e089d2012ebe37c9f44fc85ee612158300f
splat_clauses:
- key: source_rules
  text: '# Gate Level 9 Conjuration (Cleric, Sorcerer, Warlock, Wizard) Casting Time: Action Range: 60
    feet Components: V, S, M (a diamond worth 5,000+ GP) Duration: Concentration, up to 1 minute You conjure
    a portal linking an unoccupied space you can see within range to a precise location on a different
    plane of existence. The portal is a circular opening, which you can make 5 to 20 feet in diameter.
    You can orient the portal in any direction you choose. The portal lasts for the duration, and the
    portal’s destination is visible through it. The portal has a front and a back on each plane where
    it appears. Travel through the portal is possible only by moving through its front. Anything that
    does so is instantly transported to the other plane, appearing in the unoccupied space nearest to
    the portal. Deities and other planar rulers can prevent portals created by this spell from opening
    in their presence or anywhere within their domains. When you cast this spell, you can speak the name
    of a specific creature (a pseudonym, title, or nickname doesn’t work). If that creature is on a plane
    other than the one you are on, the portal opens next to the named creature and transports it to the
    nearest unoccupied space on your side of the portal. You gain no special power over the creature,
    and it is free to act as the GM deems appropriate. It might leave, attack you, or help you.'
  sha256: 097f280b66053b69908767cf9574392687bf4eff7fff80dcd1e6b6959da9c8e1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gate

<!-- SRD page 135 -->

Level 9 Conjuration (Cleric, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a diamond worth 5,000+ GP)\
Duration: Concentration, up to 1 minute\
You conjure a portal linking an unoccupied space\
you can see within range to a precise location on a\
different plane of existence. The portal is a circular\
opening, which you can make 5 to 20 feet in diam-\
eter. You can orient the portal in any direction you\
choose. The portal lasts for the duration, and the\
portal’s destination is visible through it.\
The portal has a front and a back on each plane\
where it appears. Travel through the portal is possi-\
ble only by moving through its front. Anything that\
does so is instantly transported to the other plane,\
appearing in the unoccupied space nearest to the\
portal.\
Deities and other planar rulers can prevent por-\
tals created by this spell from opening in their pres-\
ence or anywhere within their domains.\
When you cast this spell, you can speak the name\
of a specific creature (a pseudonym, title, or nick-\
name doesn’t work). If that creature is on a plane\
other than the one you are on, the portal opens\
next to the named creature and transports it to the\
nearest unoccupied space on your side of the portal.\
You gain no special power over the creature, and it\
is free to act as the GM deems appropriate. It might\
leave, attack you, or help you.\

Original layout: [[../../srd-5.2.1/pages/135|page 135]].
