---
splat_schema: 1
splat_id: f2cd7371-10dd-5eda-86b7-c87817a8a6df
splat_kind: creature
splat_key: dnd2024:creatures/shield_guardian
splat_name: Shield Guardian
splat_origin:
  key: dnd2024:reference/creature/shield_guardian
splat_sources:
- title: SRD 5.2.1
  locator: pp. 323
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: construct
  alignment: Unaligned
  movement_text: 30 ft.
  immunities_text: Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 10
  languages_text: Understands commands given in any language but can’t speak
  cr_text: 7 (XP 2,900; PB +3)
  hit_points_text: 142 (15d10 + 60)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 17
  max_hp:
    value: 142
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: bbe4cc0122ae2791cd09b6d29c33f3ef0d0990f7374fc923fcdbafe4eb897209
  source_path: builtin/srd-5.2.1/creature/shield_guardian.md
  pages:
  - 323
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Shield Guardian Large Construct, Unaligned AC 17 Initiative -1 (9) HP 142 (15d10 + 60) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 8 -1 -1 Con 18 +4 +4 Int 7 -2 -2 Wis 10 +0 +0 Cha
    3 -4 -4 Immunities Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned Senses
    Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 10 Languages Understands commands given in
    any language but can’t speak CR 7 (XP 2,900; PB +3) ### Traits Bound. The guardian is magically bound
    to an amulet. While the guardian and its amulet are on the same plane of existence, the amulet’s wearer
    can telepathically call the guardian to travel to it, and the guardian knows the distance and direction
    to the amulet. If the guardian is within 60 feet of the amulet’s wearer, half of any damage the wearer
    takes (round up) is transferred to the guardian. Regeneration. The guardian regains 10 Hit Points
    at the start of each of its turns if it has at least 1 Hit Point. Spell Storing. A spellcaster who
    wears the guardian’s amulet can cause the guardian to store one spell of level 4 or lower. To do so,
    the wearer must cast the spell on the guardian while within 5 feet of it. The spell has no effect
    but is stored within the guardian. Any previously stored spell is lost when a new spell is stored.
    The guardian can cast the spell stored with any parameters set by the original caster, requiring no
    spell components and using the caster’s spellcasting ability. The stored spell is then lost. ### Actions
    Multiattack. The guardian makes two Fist attacks. Fist. Melee Attack Roll: +7, reach 10 ft. Hit: 11
    (2d6 + 4) Bludgeoning damage plus 7 (2d6) Force damage. ### Reactions Protection. Trigger: An attack
    roll hits the wearer of the guardian’s amulet while the wearer is within 5 feet of the guardian. Response:
    The wearer gains a +5 bonus to AC, including against the triggering attack and possibly causing it
    to miss, until the start of the guardian’s next turn.'
  sha256: b40c059258a0b28801bf7440e711faafbb14822fd35ef33612b0dd4f80ca57b2
  status: needs_semantic_conversion
---

# Shield Guardian

<!-- SRD page 323 -->

Large Construct, Unaligned\
AC 17  Initiative −1 (9)\
HP 142 (15d10 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 8 −1 −1 Con 18 +4 +4\
Int 7 −2 −2 Wis 10 +0 +0 Cha 3 −4 −4\
Immunities Poison; Charmed, Exhaustion, Frightened,\
Paralyzed, Petrified, Poisoned\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 10\
Languages Understands commands given in any language\
but can’t speak\
CR 7 (XP 2,900; PB +3)\

### Traits

Bound. The guardian is magically bound to an amulet.\
While the guardian and its amulet are on the same\
plane of existence, the amulet’s wearer can telepathi-\
cally call the guardian to travel to it, and the guardian\
knows the distance and direction to the amulet. If the\
guardian is within 60 feet of the amulet’s wearer, half of\
any damage the wearer takes (round up) is transferred\
to the guardian.\
Regeneration. The guardian regains 10 Hit Points at the\
start of each of its turns if it has at least 1 Hit Point.\
Spell Storing. A spellcaster who wears the guardian’s\
amulet can cause the guardian to store one spell of\
level 4 or lower. To do so, the wearer must cast the\
spell on the guardian while within 5 feet of it. The\
spell has no effect but is stored within the guardian.\
Any previously stored spell is lost when a new spell is\
stored. The guardian can cast the spell stored with any\
parameters set by the original caster, requiring no spell\
components and using the caster’s spellcasting ability.\
The stored spell is then lost.\

### Actions

Multiattack. The guardian makes two Fist attacks.\
Fist. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6\
+ 4) Bludgeoning damage plus 7 (2d6) Force damage.\

### Reactions

Protection. Trigger: An attack roll hits the wearer of the\
guardian’s amulet while the wearer is within 5 feet of\
the guardian. Response: The wearer gains a +5 bonus\
to AC, including against the triggering attack and pos-\
sibly causing it to miss, until the start of the guardian’s\
next turn.\

Original layout: [[../../srd-5.2.1/pages/323|page 323]].
