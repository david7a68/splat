---
splat_schema: 1
splat_id: e6888c77-0bcb-516f-a305-13de712c5bd1
splat_kind: creature
splat_key: dnd2024:creatures/tarrasque
splat_name: Tarrasque
splat_origin:
  key: dnd2024:reference/creature/tarrasque
splat_sources:
- title: SRD 5.2.1
  locator: pp. 331, 332
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Unaligned
  tags_text: (Titan)
  movement_text: 60 ft., Burrow 40 ft., Climb 60 ft.
  skills_text: Perception +9
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Fire, Poison; Charmed, Deafened, Frightened, Paralyzed, Poisoned
  senses_text: Blindsight 120 ft.; Passive Perception 19
  languages_text: None
  cr_text: 30 (XP 155,000; PB +9)
  hit_points_text: 697 (34d20 + 340)
splat_stats:
  strength:
    value: 30
  save_strength:
    value: 10
  dexterity:
    value: 11
  save_dexterity:
    value: 9
  constitution:
    value: 30
  save_constitution:
    value: 10
  intelligence:
    value: 3
  save_intelligence:
    value: 5
  wisdom:
    value: 11
  save_wisdom:
    value: 9
  charisma:
    value: 11
  save_charisma:
    value: 9
  armor_class:
    value: 25
  max_hp:
    value: 697
  initiative:
    value: 18
  speed:
    value: 60
    unit: ft
  burrow_speed:
    value: 40
    unit: ft
  climb_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 516b5b36fbca2a26bc4c74cf85a83fb81c13e232429842225ec78cf0661b31cd
  source_path: builtin/srd-5.2.1/creature/tarrasque.md
  pages:
  - 331
  - 332
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Tarrasque Gargantuan Monstrosity (Titan), Unaligned AC 25 Initiative +18 (28) HP 697 (34d20
    + 340) Speed 60 ft., Burrow 40 ft., Climb 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 30 +10 +10 Dex 11
    +0 +9 Con 30 +10 +10 Int 3 -4 +5 Wis 11 +0 +9 Cha 11 +0 +9 Skills Perception +9 Resistances Bludgeoning,
    Piercing, Slashing Immunities Fire, Poison; Charmed, Deafened, Frightened, Paralyzed, Poisoned Senses
    Blindsight 120 ft.; Passive Perception 19 Languages None CR 30 (XP 155,000; PB +9) ### Traits Legendary
    Resistance (6/Day). If the tarrasque fails a saving throw, it can choose to succeed instead. Magic
    Resistance. The tarrasque has Advantage on saving throws against spells and other magical effects.
    Reflective Carapace. If the tarrasque is targeted by a Magic Missile spell or a spell that requires
    a ranged attack roll, roll 1d6. On a 1–5, the tarrasque is unaffected. On a 6, the tarrasque is unaffected
    and reflects the spell, turning the caster into the target. Siege Monster. The tarrasque deals double
    damage to objects and structures. ### Actions Multiattack. The tarrasque makes one Bite attack and
    three other attacks, using Claw or Tail in any combination. Bite. Melee Attack Roll: +19, reach 15
    ft. Hit: 36 (4d12 + 10) Piercing damage, and the target has the Grappled condition (escape DC 20).
    Until the grapple ends, the target has the Restrained condition and can’t teleport. Claw. Melee Attack
    Roll: +19, reach 15 ft. Hit: 28 (4d8 + 10) Slashing damage. Tail. Melee Attack Roll: +19, reach 30
    ft. Hit: 23 (3d8 + 10) Bludgeoning damage. If the target is a Huge or smaller creature, it has the
    Prone condition. Thunderous Bellow (Recharge 5–6). Constitution Saving Throw: DC 27, each creature
    and each object that isn’t being worn or carried in a 150-foot Cone. Failure: 78 (12d12) Thunder damage,
    and the target has the Deafened and Frightened conditions until the end of its next turn. Success:
    Half damage only. ### Bonus Actions Swallow. Strength Saving Throw: DC 27, one Large or smaller creature
    Grappled by the tarrasque (it can have up to six creatures swallowed at a time). Failure: The target
    is swallowed, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained
    conditions and can’t teleport, it has Total Cover against attacks and other effects outside the tarrasque,
    and it takes 56 (16d6) Acid damage at the start of each of the tarrasque’s turns. If the tarrasque
    takes 60 damage or more on a single turn from a creature inside it, the tarrasque must succeed on
    a DC 20 Constitution saving throw at the end of that turn or regurgitate all swallowed creatures,
    each of which falls in a space within 10 feet of the tarrasque and has the Prone condition. If the
    tarrasque dies, any swallowed creature no longer has the Restrained condition and can escape from
    the corpse using 20 feet of movement, exiting Prone. ### Legendary Actions Legendary Action Uses:
    3. Immediately after another creature’s turn, the tarrasque can expend a use to take one of the following
    actions. The tarrasque regains all expended uses at the start of each of its turns. Onslaught. The
    tarrasque moves up to half its Speed, and it makes one Claw or Tail attack. World-Shaking Movement.
    The tarrasque moves up to its Speed. At the end of this movement, the tarrasque creates an instantaneous
    shock wave in a 60-foot Emanation originating from itself. Creatures in that area lose Concentration
    and, if Medium or smaller, have the Prone condition. The tarrasque can’t take this action again until
    the start of its next turn.'
  sha256: 7c57ffc1e9edd17ac8ff78918a86f8b2d6b757b16bd38b8dfd5961c3838f5426
  status: needs_semantic_conversion
---

# Tarrasque

<!-- SRD page 331 -->

Gargantuan Monstrosity (Titan), Unaligned\
AC 25 Initiative +18 (28)\
HP 697 (34d20 + 340)\
Speed 60 ft., Burrow 40 ft., Climb 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +10 Dex 11 +0 +9 Con 30 +10 +10\
Int 3 −4 +5 Wis 11 +0 +9 Cha 11 +0 +9\
Skills Perception +9\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Fire, Poison; Charmed, Deafened, Frightened,\
Paralyzed, Poisoned\
Senses Blindsight 120 ft.; Passive Perception 19\
Languages None\
CR 30 (XP 155,000; PB +9)\

### Traits

Legendary Resistance (6/Day). If the tarrasque fails a\
saving throw, it can choose to succeed instead.\
Magic Resistance. The tarrasque has Advantage on sav-\
ing throws against spells and other magical effects.\
Reflective Carapace. If the tarrasque is targeted by a\
Magic Missile spell or a spell that requires a ranged\
attack roll, roll 1d6. On a 1–5, the tarrasque is unaf-\
fected. On a 6, the tarrasque is unaffected and reflects\
the spell, turning the caster into the target.\
Siege Monster. The tarrasque deals double damage to\
objects and structures.\

### Actions

Multiattack. The tarrasque makes one Bite attack\
and three other attacks, using Claw or Tail in any\
combination.\
Bite. Melee Attack Roll: +19, reach 15 ft. Hit: 36 (4d12\
+ 10) Piercing damage, and the target has the Grappled\

<!-- SRD page 332 -->

condition (escape DC 20). Until the grapple ends, the\
target has the Restrained condition and can’t teleport.\
Claw. Melee Attack Roll: +19, reach 15 ft. Hit: 28 (4d8\
+ 10) Slashing damage.\
Tail. Melee Attack Roll: +19, reach 30 ft. Hit: 23 (3d8\
+ 10) Bludgeoning damage. If the target is a Huge or\
smaller creature, it has the Prone condition.\
Thunderous Bellow (Recharge 5–6). Constitution Sav-\
ing Throw: DC 27, each creature and each object that\
isn’t being worn or carried in a 150-foot Cone. Failure:\
78 (12d12) Thunder damage, and the target has the\
Deafened and Frightened conditions until the end of its\
next turn. Success: Half damage only.\

### Bonus Actions

Swallow. Strength Saving Throw: DC 27, one Large or\
smaller creature Grappled by the tarrasque (it can have\
up to six creatures swallowed at a time). Failure: The\
target is swallowed, and the Grappled condition ends.\
A swallowed creature has the Blinded and Restrained\
conditions and can’t teleport, it has Total Cover against\
attacks and other effects outside the tarrasque, and it\
takes 56 (16d6) Acid damage at the start of each of the\
tarrasque’s turns.\
If the tarrasque takes 60 damage or more on a single\
turn from a creature inside it, the tarrasque must suc-\
ceed on a DC 20 Constitution saving throw at the end\
of that turn or regurgitate all swallowed creatures, each\
of which falls in a space within 10 feet of the tarrasque\
and has the Prone condition. If the tarrasque dies, any\
swallowed creature no longer has the Restrained con-\
dition and can escape from the corpse using 20 feet of\
movement, exiting Prone.\

### Legendary Actions

Legendary Action Uses: 3. Immediately after another\
creature’s turn, the tarrasque can expend a use to take\
one of the following actions. The tarrasque regains all\
expended uses at the start of each of its turns.\
Onslaught. The tarrasque moves up to half its Speed,\
and it makes one Claw or Tail attack.\
World-Shaking Movement. The tarrasque moves up to\
its Speed. At the end of this movement, the tarrasque\
creates an instantaneous shock wave in a 60-foot Em-\
anation originating from itself. Creatures in that area\
lose Concentration and, if Medium or smaller, have the\
Prone condition. The tarrasque can’t take this action\
again until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/331|page 331]], [[../../srd-5.2.1/pages/332|page 332]].
