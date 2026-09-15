---
splat_schema: 1
splat_id: 38bc921e-b3ab-5d83-a3d6-c2bccda83ee3
splat_kind: spell
splat_key: dnd2024:spells/detect_thoughts
splat_name: Detect Thoughts
splat_origin:
  key: dnd2024:reference/spell/detect_thoughts
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - bard
  - sorcerer
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
    mode: self
  material:
    description: 1 Copper Piece
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/detect_thoughts
  implementation: dnd2024:spell_detect_thoughts
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    activation:
      at_cast: choose_one_option
      later:
        economy: magic_action
        timing: caster_later_turn
        options:
        - sense_thoughts
        - read_thoughts
    sense_thoughts:
      range_feet: 30
      eligible:
        any:
        - knows_language
        - telepathic
      output: thinking_creature_presence
      read_content: false
    read_thoughts:
      targets: 1
      range_feet: 30
      eligible:
        any:
        - visible
        - detected_with_sense_thoughts
      output: foremost_current_thought
      no_language_and_not_telepathic_output: nothing
    probe:
      economy: magic_action
      timing: next_caster_turn_after_read
      target: read_target
      save:
        ability: wisdom
        dc: spell_save_dc
      failure_output:
      - reasoning
      - emotions
      - prominent_concern
      success: end_spell
      either_outcome: target_knows_probe
    resistance_after_probe:
      until: attention_shifts_to_other_target
      economy: action
      timing: target_turn
      check:
        ability: intelligence
        skill: arcana
        dc: spell_save_dc
      success: end_spell
    barriers:
    - material: stone
      thickness_inches: 12
    - material: dirt
      thickness_inches: 12
    - material: wood
      thickness_inches: 12
    - material: metal
      thickness_inches: 1
    - material: lead
      thickness: any_sheet
    barrier_scope: spell
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_detect_thoughts
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    option:
      type: choice
      required: true
    target:
      required: false
      type: participant
    thought_content:
      required: false
      type: choice
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - information
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - information
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3b91c8e3dc1fd6941751524768650c8b6cdb59d68bf32e0980611918d2734bb6
  source_path: builtin/srd-5.2.1/spell/detect_thoughts.md
  pages:
  - 123
  engine_gaps: &id001
  - dnd2024:spell_detect_thoughts
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: b30c1ccc8f6fb277332fc6b1985da4a7ee7133ebde9a1deb50007c8aa796252f
splat_clauses:
- key: source_rules
  text: '# Detect Thoughts Level 2 Divination (Bard, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S, M (1 Copper Piece) Duration: Concentration, up to 1 minute You activate one of the
    effects below. Until the spell ends, you can activate either effect as a Magic action on your later
    turns. Sense Thoughts. You sense the presence of thoughts within 30 feet of yourself that belong to
    creatures that know languages or are telepathic. You don’t read the thoughts, but you know that a
    thinking creature is present. The spell is blocked by 1 foot of stone, dirt, or wood; 1 inch of metal;
    or a thin sheet of lead. Read Thoughts. Target one creature you can see within 30 feet of yourself
    or one creature within 30 feet of yourself that you detected with the Sense Thoughts option. You learn
    what is most on the target’s mind right now. If the target doesn’t know any languages and isn’t telepathic,
    you learn nothing. As a Magic action on your next turn, you can try to probe deeper into the target’s
    mind. If you probe deeper, the target makes a Wisdom saving throw. On a failed save, you discern the
    target’s reasoning, emotions, and something that looms large in its mind (such as a worry, love, or
    hate). On a successful save, the spell ends. Either way, the target knows that you are probing into
    its mind, and until you shift your attention away from the target’s mind, the target can take an action
    on its turn to make an Intelligence (Arcana) check against your spell save DC, ending the spell on
    a success.'
  sha256: 2bcd8f8cb0960d0c848973a066ccabc674f1f6e889653de0e7b0f34e9d45f7dc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Detect Thoughts

<!-- SRD page 123 -->

Level 2 Divination (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (1 Copper Piece)\
Duration: Concentration, up to 1 minute\
You activate one of the effects below. Until the spell\
ends, you can activate either effect as a Magic action\
on your later turns.\
Sense Thoughts. You sense the presence of\
thoughts within 30 feet of yourself that belong to\
creatures that know languages or are telepathic.\
You don’t read the thoughts, but you know that a\
thinking creature is present.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\
Read Thoughts. Target one creature you can see\
within 30 feet of yourself or one creature within\
30 feet of yourself that you detected with the Sense\
Thoughts option. You learn what is most on the tar-\
get’s mind right now. If the target doesn’t know any\
languages and isn’t telepathic, you learn nothing.\
As a Magic action on your next turn, you can try\
to probe deeper into the target’s mind. If you probe\
deeper, the target makes a Wisdom saving throw.\
On a failed save, you discern the target’s reasoning,\
emotions, and something that looms large in its\
mind (such as a worry, love, or hate). On a successful\
save, the spell ends. Either way, the target knows\
that you are probing into its mind, and until you\
shift your attention away from the target’s mind,\
the target can take an action on its turn to make an\
Intelligence (Arcana) check against your spell save\
DC, ending the spell on a success.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].
