---
splat_schema: 1
splat_id: 2618e55d-2863-5233-8639-f3def3fded3d
splat_kind: spell
splat_key: dnd2024:spells/prismatic_wall
splat_name: Prismatic Wall
splat_origin:
  key: dnd2024:reference/spell/prismatic_wall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 155
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: abjuration
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: 6a2938d3e6e4be83cbdab37cfd8df25b1b0456ae29b0ce3fc904cde2cedbd37e
  source_path: builtin/srd-5.2.1/spell/prismatic_wall.md
  pages:
  - 155
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Prismatic Wall Level 9 Abjuration (Bard, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: 10 minutes A shimmering, multicolored plane of light forms a vertical opaque wall—up
    to 90 feet long, 30 feet high, and 1 inch thick—centered on a point within range. Alternatively, you
    shape the wall into a globe up to 30 feet in diameter centered on a point within range. The wall lasts
    for the duration. If you position the wall in a space occupied by a creature, the spell ends instantly
    without effect. The wall sheds Bright Light within 100 feet and Dim Light for an additional 100 feet.
    You and creatures you designate when you cast the spell can pass through and be near the wall without
    harm. If another creature that can see the wall moves within 20 feet of it or starts its turn there,
    the creature must succeed on a Constitution saving throw or have the Blinded condition for 1 minute.
    The wall consists of seven layers, each with a different color. When a creature reaches into or passes
    through the wall, it does so one layer at a time through all the layers. Each layer forces the creature
    to make a Dexterity saving throw or be affected by that layer’s properties as described in the Prismatic
    Layers table. The wall, which has AC 10, can be destroyed one layer at a time, in order from red to
    violet, by means specific to each layer. If a layer is destroyed, it is gone for the duration. Antimagic
    Field has no effect on the wall, and Dispel Magic can affect only the violet layer. Prismatic Layers
    Order Effects 1 Red. Failed Save: 12d6 Fire damage. Successful Save: Half as much damage. Additional
    Effects: Nonmagical ranged attacks can’t pass through this layer, which is destroyed if it takes at
    least 25 Cold damage. 2 Orange. Failed Save: 12d6 Acid damage. Successful Save: Half as much damage.
    Additional Effects: Magical ranged attacks can’t pass through this layer, which is destroyed by a
    strong wind (such as the one created by Gust of Wind). 3 Yellow. Failed Save: 12d6 Lightning damage.
    Successful Save: Half as much damage. Additional Effects: The layer is destroyed if it takes at least
    60 Force damage. 4 Green. Failed Save: 12d6 Poison damage. Successful Save: Half as much damage. Additional
    Effects: A Passwall spell, or another spell of equal or greater level that can open a portal on a
    solid surface, destroys this layer. 5 Blue. Failed Save: 12d6 Cold damage. Successful Save: Half as
    much damage. Additional Effects: The layer is destroyed if it takes at least 25 Fire damage. 6 Indigo.
    Failed Save: The target has the Restrained condition and makes a Constitution saving throw at the
    end of each of its turns. If it successfully saves three times, the condition ends. If it fails three
    times, it has the Petrified condition until it is freed by an effect like the Greater Restoration
    spell. The successes and failures needn’t be consecutive; keep track of both until the target collects
    three of a kind. Additional Effects: Spells can’t be cast through this layer, which is destroyed by
    Bright Light shed by the Daylight spell. 7 Violet. Failed Save: The target has the Blinded condition
    and makes a Wisdom saving throw at the start of your next turn. On a successful save, the condition
    ends. On a failed save, the condition ends, and the creature teleports to another plane of existence
    (GM’s choice). Additional Effects: This layer is destroyed by Dispel Magic.'
  sha256: 4d0b20a59c73c0b0f5538918b22b2557a4c926e6d512bfafa01759c72d91fb26
  status: needs_semantic_conversion
---

# Prismatic Wall

<!-- SRD page 155 -->

Level 9 Abjuration (Bard, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 10 minutes\
A shimmering, multicolored plane of light forms\
a vertical opaque wall—up to 90 feet long, 30 feet\
high, and 1 inch thick—centered on a point within\
range. Alternatively, you shape the wall into a globe\
up to 30 feet in diameter centered on a point within\
range. The wall lasts for the duration. If you posi-\
tion the wall in a space occupied by a creature, the\
spell ends instantly without effect.\
The wall sheds Bright Light within 100 feet and\
Dim Light for an additional 100 feet. You and crea-\
tures you designate when you cast the spell can\
pass through and be near the wall without harm. If\
another creature that can see the wall moves within\
20 feet of it or starts its turn there, the creature\
must succeed on a Constitution saving throw or\
have the Blinded condition for 1 minute.\
The wall consists of seven layers, each with a\
different color. When a creature reaches into or\
passes through the wall, it does so one layer at a\
time through all the layers. Each layer forces the\
creature to make a Dexterity saving throw or be af-\
fected by that layer’s properties as described in the\
Prismatic Layers table.\
The wall, which has AC 10, can be destroyed one\
layer at a time, in order from red to violet, by means\
specific to each layer. If a layer is destroyed, it is\
gone for the duration. Antimagic Field has no effect\
on the wall, and Dispel Magic can affect only the vio-\
let layer.\
Prismatic Layers\
Order Effects\
1 Red. Failed Save: 12d6 Fire damage. Successful\
Save: Half as much damage. Additional Effects:\
Nonmagical ranged attacks can’t pass through\
this layer, which is destroyed if it takes at\
least 25 Cold damage.\
2 Orange. Failed Save: 12d6 Acid damage. Suc-\
cessful Save: Half as much damage. Additional\
Effects: Magical ranged attacks can’t pass\
through this layer, which is destroyed by a\
strong wind (such as the one created by Gust\
of Wind).\
3 Yellow. Failed Save: 12d6 Lightning damage.\
Successful Save: Half as much damage. Addi-\
tional Effects: The layer is destroyed if it takes\
at least 60 Force damage.\
4 Green. Failed Save: 12d6 Poison damage. Suc-\
cessful Save: Half as much damage. Additional\
Effects: A Passwall spell, or another spell of\
equal or greater level that can open a portal\
on a solid surface, destroys this layer.\
5 Blue. Failed Save: 12d6 Cold damage. Success-\
ful Save: Half as much damage. Additional Ef-\
fects: The layer is destroyed if it takes at least\
25 Fire damage.\
6 Indigo. Failed Save: The target has the Re-\
strained condition and makes a Constitution\
saving throw at the end of each of its turns. If\
it successfully saves three times, the condition\
ends. If it fails three times, it has the Petrified\
condition until it is freed by an effect like the\
Greater Restoration spell. The successes and\
failures needn’t be consecutive; keep track of\
both until the target collects three of a kind.\
Additional Effects: Spells can’t be cast through\
this layer, which is destroyed by Bright Light\
shed by the Daylight spell.\
7 Violet. Failed Save: The target has the Blinded\
condition and makes a Wisdom saving throw\
at the start of your next turn. On a successful\
save, the condition ends. On a failed save, the\
condition ends, and the creature teleports\
to another plane of existence (GM’s choice).\
Additional Effects: This layer is destroyed by\
Dispel Magic.\

Original layout: [[../../srd-5.2.1/pages/155|page 155]].
