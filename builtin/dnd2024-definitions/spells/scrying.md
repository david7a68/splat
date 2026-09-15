---
splat_schema: 1
splat_id: 8174ca4b-590c-5811-a154-1f6821ae809c
splat_kind: spell
splat_key: dnd2024:spells/scrying
splat_name: Scrying
splat_origin:
  key: dnd2024:reference/spell/scrying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: divination
  classes:
  - bard
  - cleric
  - druid
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: 10 minutes
  ritual: false
  range:
    mode: self
  material:
    description: a focus worth 1,000+ GP, such as a crystal ball, mirror, or water-filled font
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/scrying
  implementation: dnd2024:spell_scrying
  parameters:
    targeting:
      options:
      - kind: creature
        same_plane: true
      - kind: location
        previously_seen: true
    creature_save:
      ability: wisdom
      knowledge_modifier:
        secondhand: 5
        firsthand: 0
        extensive: -5
      connection_modifier:
        none: 0
        likeness: -2
        possession: -4
        body_part_hair_or_nail: -10
      combination: knowledge_plus_selected_connection
      target_awareness: unease_without_knowing_save_source
      success:
        no_effect: true
        cannot_retarget_hours: 24
      failure: create_sensor
      connection_selection:
        type: choice
        chooser: caster
        choices: available_physical_connections
        count: 1
    sensor:
      invisible: true
      intangible: true
      channels:
      - vision
      - hearing
      perspective: sensor_location
      creature_mode:
        distance_max_feet: 10
        moves_with_target: true
      location_mode:
        location: chosen_location
        moves: false
        save: none
      visible_appearance: fist_sized_luminous_orb
      duration: spell_duration
    spell_lifetime:
      concentration: true
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_scrying
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    locations:
      type: location_list
      required: true
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
  - planar_travel
  - perception
splat_conversion:
  status: specified
  category: spell
  source_sha256: 83a873ea9baf07a61bd7a984e871fe110cf0f822bd02f11fd5c9ed456da8ad23
  source_path: builtin/srd-5.2.1/spell/scrying.md
  pages:
  - 160
  engine_gaps: &id001
  - dnd2024:spell_scrying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 493ac785d35f77d4b50fff393e9620187edf5164756dbd379ade657d741391d1
splat_clauses:
- key: source_rules
  text: '# Scrying Level 5 Divination (Bard, Cleric, Druid, Warlock, Wizard) Casting Time: 10 minutes
    Range: Self Components: V, S, M (a focus worth 1,000+ GP, such as a crystal ball, mirror, or water-filled
    font) Duration: Concentration, up to 10 minutes You can see and hear a creature you choose that is
    on the same plane of existence as you. The target makes a Wisdom saving throw, which is modified (see
    the tables below) by how well you know the target and the sort of physical connection you have to
    it. The target doesn’t know what it is making the save against, only that it feels uneasy. Your Knowledge
    of the Target Is … Save Modifier Secondhand (heard of the target) +5 Firsthand (met the target) +0
    Extensive (know the target well) -5 You Have the Target’s … Save Modifier Picture or other likeness
    -2 Garment or other possession -4 Body part, lock of hair, or bit of nail -10 On a successful save,
    the target isn’t affected, and you can’t use this spell on it again for 24 hours. On a failed save,
    the spell creates an Invisible, intangible sensor within 10 feet of the target. You can see and hear
    through the sensor as if you were there. The sensor moves with the target, remaining within 10 feet
    of it for the duration. If something can see the sensor, it appears as a luminous orb about the size
    of your fist. Instead of targeting a creature, you can target a location you have seen. When you do
    so, the sensor appears at that location and doesn’t move.'
  sha256: 9b083b2cdc8ff65d28f14da589a192f22b2df0c4e0bbe2c2378e4c7952625e17
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scrying

<!-- SRD page 160 -->

Level 5 Divination (Bard, Cleric, Druid, Warlock,\
Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (a focus worth 1,000+ GP, such as\
a crystal ball, mirror, or water-filled font)\
Duration: Concentration, up to 10 minutes\
You can see and hear a creature you choose that is\
on the same plane of existence as you. The target\
makes a Wisdom saving throw, which is modified\
(see the tables below) by how well you know the\
target and the sort of physical connection you have\
to it. The target doesn’t know what it is making the\
save against, only that it feels uneasy.\
Your Knowledge of the Target Is … Save Modifier\
Secondhand (heard of the target) +5\
Firsthand (met the target) +0\
Extensive (know the target well) −5\
You Have the Target’s … Save Modifier\
Picture or other likeness −2\
Garment or other possession −4\
Body part, lock of hair, or bit of nail −10\
On a successful save, the target isn’t affected, and\
you can’t use this spell on it again for 24 hours.\
On a failed save, the spell creates an Invisible,\
intangible sensor within 10 feet of the target. You\
can see and hear through the sensor as if you were\
there. The sensor moves with the target, remaining\
within 10 feet of it for the duration. If something\
can see the sensor, it appears as a luminous orb\
about the size of your fist.\
Instead of targeting a creature, you can target a\
location you have seen. When you do so, the sensor\
appears at that location and doesn’t move.\

Original layout: [[../../srd-5.2.1/pages/160|page 160]].
