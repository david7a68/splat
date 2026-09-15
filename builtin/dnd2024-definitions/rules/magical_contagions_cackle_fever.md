---
splat_schema: 1
splat_id: 1afbbc28-9002-5d09-b54b-978427fb3c01
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_cackle_fever
splat_name: Cackle Fever
splat_origin:
  key: dnd2024:reference/pages/194
splat_sources:
- title: SRD 5.2.1
  locator: pp. 194
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions_cackle_fever
  parameters:
    contagion: cackle_fever
    eligible_type: humanoid
    immune_species:
    - gnome
    onset_delay:
      dice:
        count: 1
        sides: 4
        add: 0
      unit: days
    onset_exhaustion:
      levels: 1
      duration: until_contagion_ends
    laughter:
      requires_exhaustion: true
      trigger: damage_taken
      excluded_damage_types:
      - psychic
      save:
        ability: constitution
        dc: 13
      failure:
        damage:
          dice:
            count: 1
            sides: 10
            add: 0
          average: 5
          type: psychic
        condition: incapacitated
      repeat_save:
        event: affected_turn.ended
        success: end_laughter
      automatic_success_after:
        value: 1
        unit: minute
    recovery:
      event: long_rest.finished
      save:
        ability: constitution
        dc: 13
      success_count: 3
      consecutive_required: false
      success_threshold: end_contagion
      subsequent_immunity:
        value: 1
        unit: year
    spread:
      event: susceptible_turn.started
      area:
        shape: emanation
        radius_feet: 10
        origin: infected_creature
      save:
        ability: constitution
        dc: 10
      failure: infect
      success_immunity:
        scope: this_infected_source
        duration:
          value: 24
          unit: hours
  implementation: dnd2024:rules_magical_contagions_cackle_fever
splat_contracts:
- id: dnd2024:rules_magical_contagions_cackle_fever
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events:
  - turn.started
  - turn.ended
  - damage.applied
  - rest.long_completed
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 194
  engine_gaps: &id001
  - dnd2024:rules_magical_contagions_cackle_fever
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: d80bf6a93cfaa74bd167ab3715f7bfe6defdae89cc9cc5affa5df0f808226472
  parent_name: Magical Contagions
  line_span:
    first: 9527
    count: 32
splat_clauses:
- key: source_rules
  text: '# Cackle Fever Magical Contagion Cheaply made potions and elixirs are sometimes tainted by Cackle
    Fever, which affects Humanoids only (gnomes are strangely immune). A creature suffers the following
    effects 1d4 days after infection: Fever. The creature gains 1 Exhaustion level, which lasts until
    the contagion ends on the creature. Uncontrollable Laughter. While the creature has the Exhaustion
    condition, the creature makes a DC 13 Constitution saving throw each time it takes damage other than
    Psychic damage. On a failed save, the creature takes 5 (1d10) Psychic damage and has the Incapacitated
    condition as it laughs uncontrollably. At the end of each of its turns, the creature repeats the save,
    ending the effect on itself on a success. After 1 minute, it succeeds automatically. Fighting the
    Contagion. At the end of each Long Rest, an infected creature makes a DC 13 Constitution saving throw.
    After the creature succeeds on three of these saves, the contagion ends on it, and the creature is
    immune to Cackle Fever for 1 year. Spreading the Contagion. Any Humanoid (other than a gnome) that
    starts its turn within a 10-foot Emanation originating from a creature infected with Cackle Fever
    must succeed on a DC 10 Constitution saving throw or also become infected with the contagion. On a
    successful save, the Humanoid can’t catch the contagion from that particular infected creature for
    the next 24 hours.'
  sha256: 8c18d49bd59f81605f433fe7079dc0215b54ca6a368cd65effb6a8b35af08003
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cackle Fever

<!-- SRD page 194 -->

Magical Contagion\
Cheaply made potions and elixirs are sometimes\
tainted by Cackle Fever, which affects Humanoids\
only (gnomes are strangely immune). A crea-\
ture suffers the following effects 1d4 days after\
infection:\
Fever. The creature gains 1 Exhaustion level, which\
lasts until the contagion ends on the creature.\
Uncontrollable Laughter. While the creature has\
the Exhaustion condition, the creature makes\
a DC 13 Constitution saving throw each time it\
takes damage other than Psychic damage. On a\
failed save, the creature takes 5 (1d10) Psychic\
damage and has the Incapacitated condition as\
it laughs uncontrollably. At the end of each of its\
turns, the creature repeats the save, ending the\
effect on itself on a success. After 1 minute, it suc-\
ceeds automatically.\
Fighting the Contagion. At the end of each Long\
Rest, an infected creature makes a DC 13 Constitu-\
tion saving throw. After the creature succeeds on\
three of these saves, the contagion ends on it, and\
the creature is immune to Cackle Fever for 1 year.\
Spreading the Contagion. Any Humanoid (other\
than a gnome) that starts its turn within a 10-foot\
Emanation originating from a creature infected\
with Cackle Fever must succeed on a DC 10 Consti-\
tution saving throw or also become infected with\
the contagion. On a successful save, the Humanoid\
can’t catch the contagion from that particular in-\
fected creature for the next 24 hours.\
