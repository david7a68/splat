---
splat_schema: 1
splat_id: b5a5c587-b6b9-5e9e-9dd9-ef58d6d3321d
splat_kind: creature
splat_key: dnd2024:creatures/hydra
splat_name: Hydra
splat_origin:
  key: dnd2024:reference/creature/hydra
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Swim 40 ft.
  skills_text: Perception +6
  immunities_text: Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 16
  languages_text: None
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 184 (16d12 + 80)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 184
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_traits:
  condition_immunities:
    rule: dnd2024:condition_immunities
    parameters:
      conditions:
      - blinded
      - charmed
      - deafened
      - frightened
      - stunned
      - unconscious
  hold_breath:
    rule: dnd2024:hold_breath
    parameters:
      duration:
        value: 1
        unit: hour
  heads:
    rule: dnd2024:hydra_heads_2024
    parameters:
      initial_heads: 5
      damage_threshold_per_turn: 25
      heads_lost_on_threshold: 1
      death_when_heads: 0
      regrow_at: owner_turn_end
      regrow_per_lost_head: 2
      lost_head_window: since_last_owner_turn
      regrowth_requires_living_head: true
      regrowth_prevented_by: fire_damage_since_last_owner_turn
      healing_per_regrowth_event: 20
      extra_reactions_per_head_beyond_first: 1
      extra_reactions_allowed: opportunity_attack
      extra_reactions_refresh: owner_turn_start
      reaction_capacity_recomputed_at: owner_turn_start
splat_actions:
  bite:
    name: Bite
    activation:
      economy: action
      category: attack
      timing: on_turn
      cost_at: commit
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        allow_self: false
        range:
          feet: 10
    rolls:
      attack:
        expression: 1d20
        modifier: 8
        scope: attack
      damage:
        expression: 1d10 + 5
        scope: attack
    steps:
    - key: attack
      operation: dnd2024:attack
      kind: melee_weapon
      target: chosen
      roll: attack
      on_hit:
        damage:
        - type: piercing
          amount:
            roll: damage
  multiattack:
    name: Multiattack
    activation:
      economy: action
      category: attack
      timing: on_turn
      cost_at: commit
    steps:
    - key: heads
      operation: dnd2024:hydra_heads_2024
      parameters:
        mode: multiattack
        attack: bite
        count: living_heads
        pay_attack_economy_again: false
splat_conversion:
  status: specified
  category: creature
  source_sha256: 96505c3a52429bac9d95003be408df5a3deb0dd3d4abdf1745eb6afeda326843
  source_path: builtin/srd-5.2.1/creature/hydra.md
  pages:
  - 300
  engine_gaps: &id001
  - dnd2024:hydra_heads_2024
  - dnd2024:hold_breath
  conversion_gaps: []
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Hydra Huge Monstrosity, Unaligned AC 15 Initiative +4 (14) HP 184 (16d12 + 80) Speed 40 ft.,
    Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 12 +1 +1 Con 20 +5 +5 Int 2 -4 -4 Wis 10 +0
    +0 Cha 7 -2 -2 Skills Perception +6 Immunities Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious
    Senses Darkvision 60 ft.; Passive Perception 16 Languages None CR 8 (XP 3,900; PB +3) ### Traits Hold
    Breath. The hydra can hold its breath for 1 hour. Multiple Heads. The hydra has five heads. Whenever
    the hydra takes 25 damage or more on a single turn, one of its heads dies. The hydra dies if all its
    heads are dead. At the end of each of its turns when it has at least one living head, the hydra grows
    two heads for each of its heads that died since its last turn, unless it has taken Fire damage since
    its last turn. The hydra regains 20 Hit Points when it grows new heads. Reactive Heads. For each head
    the hydra has beyond one, it gets an extra Reaction that can be used only for Opportunity Attacks.
    ### Actions Multiattack. The hydra makes as many Bite attacks as it has heads. Bite. Melee Attack
    Roll: +8, reach 10 ft. Hit: 10 (1d10 + 5) Piercing damage.'
  sha256: faa03386c63f4afa2c6a7a294353f6eb7f70756e81af2a0b15829078a63ddc91
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hydra

<!-- SRD page 300 -->

Huge Monstrosity, Unaligned\
AC 15 Initiative +4 (14)\
HP 184 (16d12 + 80)\
Speed 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 12 +1 +1 Con 20 +5 +5\
Int 2 −4 −4 Wis 10 +0 +0 Cha 7 −2 −2\
Skills Perception +6\
Immunities Blinded, Charmed, Deafened, Frightened,\
Stunned, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 16\
Languages None\
CR 8 (XP 3,900; PB +3)\

### Traits

Hold Breath. The hydra can hold its breath for 1 hour.\
Multiple Heads. The hydra has five heads. Whenever\
the hydra takes 25 damage or more on a single turn,\
one of its heads dies. The hydra dies if all its heads are\
dead. At the end of each of its turns when it has at least\
one living head, the hydra grows two heads for each of\
its heads that died since its last turn, unless it has taken\
Fire damage since its last turn. The hydra regains 20 Hit\
Points when it grows new heads.\
Reactive Heads. For each head the hydra has beyond\
one, it gets an extra Reaction that can be used only for\
Opportunity Attacks.\

### Actions

Multiattack. The hydra makes as many Bite attacks as it\
has heads.\
Bite. Melee Attack Roll: +8, reach 10 ft. Hit: 10 (1d10\
+ 5) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/300|page 300]].
