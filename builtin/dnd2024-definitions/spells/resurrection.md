---
splat_schema: 1
splat_id: edf1ef04-81a2-55ba-98f0-e742a7834c1f
splat_kind: spell
splat_key: dnd2024:spells/resurrection
splat_name: Resurrection
splat_origin:
  key: dnd2024:reference/spell/resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158, 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: necromancy
  classes:
  - bard
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: a diamond worth 1,000+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: cf628b30365e82d88b812633179595a2ae22e2210427c02288bb40d7419c5177
  source_path: builtin/srd-5.2.1/spell/resurrection.md
  pages:
  - 158
  - 159
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Resurrection Level 7 Necromancy (Bard, Cleric) Casting Time: 1 hour Range: Touch Components:
    V, S, M (a diamond worth 1,000+ GP, which the spell consumes) Duration: Instantaneous With a touch,
    you revive a dead creature that has been dead for no more than a century, didn’t die of old age, and
    wasn’t Undead when it died. The creature returns to life with all its Hit Points. This spell also
    neutralizes any poisons that affected the creature at the time of death. This spell closes all mortal
    wounds and restores any missing body parts. Coming back from the dead is an ordeal. The target takes
    a -4 penalty to D20 Tests. Every time the target finishes a Long Rest, the penalty is reduced by 1
    until it becomes 0. Casting this spell to revive a creature that has been dead for 365 days or longer
    taxes you. Until you finish a Long Rest, you can’t cast spells again, and you have Disadvantage on
    D20 Tests.'
  sha256: 1efe5b7d76b87def5469ec3b4a4e13a62a951df37b89977377107d2f3783b28f
  status: needs_semantic_conversion
---

# Resurrection

<!-- SRD page 158 -->

Level 7 Necromancy (Bard, Cleric)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 1,000+ GP,\
which the spell consumes)\
Duration: Instantaneous\

<!-- SRD page 159 -->

With a touch, you revive a dead creature that has\
been dead for no more than a century, didn’t die of\
old age, and wasn’t Undead when it died.\
The creature returns to life with all its Hit Points.\
This spell also neutralizes any poisons that affected\
the creature at the time of death. This spell closes\
all mortal wounds and restores any missing body\
parts.\
Coming back from the dead is an ordeal. The tar-\
get takes a −4 penalty to D20 Tests. Every time the\
target finishes a Long Rest, the penalty is reduced\
by 1 until it becomes 0.\
Casting this spell to revive a creature that has\
been dead for 365 days or longer taxes you. Until\
you finish a Long Rest, you can’t cast spells again,\
and you have Disadvantage on D20 Tests.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]], [[../../srd-5.2.1/pages/159|page 159]].
