---
splat_schema: 1
splat_id: 0ceb28b0-5941-5176-852a-7ff58d127cf5
splat_kind: spell
splat_key: dnd2024:spells/glyph_of_warding
splat_name: Glyph of Warding
splat_origin:
  key: dnd2024:reference/spell/glyph_of_warding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 136, 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - bard
  - cleric
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled or triggered
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: powdered diamond worth 200+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 7dc0e370c978e5fc5006584603ff9d8b0b1ab16fc515cdf82cc615009b197a93
  source_path: builtin/srd-5.2.1/spell/glyph_of_warding.md
  pages:
  - 136
  - 137
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Glyph of Warding Level 3 Abjuration (Bard, Cleric, Wizard) Casting Time: 1 hour Range: Touch
    Components: V, S, M (powdered diamond worth 200+ GP, which the spell consumes) Duration: Until dispelled
    or triggered You inscribe a glyph that later unleashes a magical effect. You inscribe it either on
    a surface (such as a table or a section of floor) or within an object that can be closed (such as
    a book or chest) to conceal the glyph. The glyph can cover an area no larger than 10 feet in diameter.
    If the surface or object is moved more than 10 feet from where you cast this spell, the glyph is broken,
    and the spell ends without being triggered. The glyph is nearly imperceptible and requires a successful
    Wisdom (Perception) check against your spell save DC to notice. When you inscribe the glyph, you set
    its trigger and choose whether it’s an explosive rune or a spell glyph, as explained below. Set the
    Trigger. You decide what triggers the glyph when you cast the spell. For glyphs inscribed on a surface,
    common triggers include touching or stepping on the glyph, removing another object covering it, or
    approaching within a certain distance of it. For glyphs inscribed within an object, common triggers
    include opening that object or seeing the glyph. Once a glyph is triggered, this spell ends. You can
    refine the trigger so that only creatures of certain types activate it (for example, the glyph could
    be set to affect Aberrations). You can also set conditions for creatures that don’t trigger the glyph,
    such as those who say a certain password. Explosive Rune. When triggered, the glyph erupts with magical
    energy in a 20-foot-radius Sphere centered on the glyph. Each creature in the area makes a Dexterity
    saving throw. A creature takes 5d8 Acid, Cold, Fire, Lightning, or Thunder damage (your choice when
    you create the glyph) on a failed save or half as much damage on a successful one. Spell Glyph. You
    can store a prepared spell of level 3 or lower in the glyph by casting it as part of creating the
    glyph. The spell must target a single creature or an area. The spell being stored has no immediate
    effect when cast in this way. When the glyph is triggered, the stored spell takes effect. If the spell
    has a target, it targets the creature that triggered the glyph. If the spell affects an area, the
    area is centered on that creature. If the spell summons Hostile creatures or creates harmful objects
    or traps, they appear as close as possible to the intruder and attack it. If the spell requires Concentration,
    it lasts until the end of its full duration. Using a Higher-Level Spell Slot. The damage of an explosive
    rune increases by 1d8 for each spell slot level above 3. If you create a spell glyph, you can store
    any spell of up to the same level as the spell slot you use for the Glyph of Warding.'
  sha256: 7c42defebfe87c0713d971350c202d478915fb6b7fec2ff0f9ee73345bb7ea29
  status: needs_semantic_conversion
---

# Glyph of Warding

<!-- SRD page 136 -->

Level 3 Abjuration (Bard, Cleric, Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (powdered diamond worth 200+\
GP, which the spell consumes)\
Duration: Until dispelled or triggered\
You inscribe a glyph that later unleashes a magical\
effect. You inscribe it either on a surface (such as a\
table or a section of floor) or within an object that\
can be closed (such as a book or chest) to conceal\
the glyph. The glyph can cover an area no larger\
than 10 feet in diameter. If the surface or object is\
moved more than 10 feet from where you cast this\
spell, the glyph is broken, and the spell ends with-\
out being triggered.\
The glyph is nearly imperceptible and requires a\
successful Wisdom (Perception) check against your\
spell save DC to notice.\
When you inscribe the glyph, you set its trigger\
and choose whether it’s an explosive rune or a spell\
glyph, as explained below.\
Set the Trigger. You decide what triggers the\
glyph when you cast the spell. For glyphs inscribed\
on a surface, common triggers include touching or\
stepping on the glyph, removing another object cov-\
ering it, or approaching within a certain distance of\
it. For glyphs inscribed within an object, common\
triggers include opening that object or seeing the\
glyph. Once a glyph is triggered, this spell ends.\
You can refine the trigger so that only creatures\
of certain types activate it (for example, the glyph\
could be set to affect Aberrations). You can also set\
conditions for creatures that don’t trigger the glyph,\
such as those who say a certain password.\
Explosive Rune. When triggered, the glyph erupts\
with magical energy in a 20-foot-radius Sphere cen-\
tered on the glyph. Each creature in the area makes\
a Dexterity saving throw. A creature takes 5d8 Acid,\
Cold, Fire, Lightning, or Thunder damage (your\

<!-- SRD page 137 -->

choice when you create the glyph) on a failed save\
or half as much damage on a successful one.\
Spell Glyph. You can store a prepared spell of\
level 3 or lower in the glyph by casting it as part of\
creating the glyph. The spell must target a single\
creature or an area. The spell being stored has no\
immediate effect when cast in this way.\
When the glyph is triggered, the stored spell\
takes effect. If the spell has a target, it targets the\
creature that triggered the glyph. If the spell affects\
an area, the area is centered on that creature. If the\
spell summons Hostile creatures or creates harmful\
objects or traps, they appear as close as possible to\
the intruder and attack it. If the spell requires Con-\
centration, it lasts until the end of its full duration.\
Using a Higher-Level Spell Slot. The damage of\
an explosive rune increases by 1d8 for each spell\
slot level above 3. If you create a spell glyph, you can\
store any spell of up to the same level as the spell\
slot you use for the Glyph of Warding.\

Original layout: [[../../srd-5.2.1/pages/136|page 136]], [[../../srd-5.2.1/pages/137|page 137]].
