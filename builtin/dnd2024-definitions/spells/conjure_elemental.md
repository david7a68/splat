---
splat_schema: 1
splat_id: bcfa9253-12b1-553a-bc7c-eaf05c6eb8ec
splat_kind: spell
splat_key: dnd2024:spells/conjure_elemental
splat_name: Conjure Elemental
splat_origin:
  key: dnd2024:reference/spell/conjure_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: conjuration
  classes:
  - druid
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: 8c241874cfeb5d3411ddf80fef35474191de3f10ffb0adbb90f80f204bc44cd7
  source_path: builtin/srd-5.2.1/spell/conjure_elemental.md
  pages:
  - 118
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Conjure Elemental Level 5 Conjuration (Druid, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Concentration, up to 10 minutes You conjure a Large, intangible spirit from the Elemental
    Planes that appears in an unoccupied space within range. Choose the spirit’s element, which determines
    its damage type: air (Lightning), earth (Thunder), fire (Fire), or water (Cold). The spirit lasts
    for the duration. Whenever a creature you can see enters the spirit’s space or starts its turn within
    5 feet of the spirit, you can force that creature to make a Dexterity saving throw if the spirit has
    no creature Restrained. On failed save, the target takes 8d8 damage of the spirit’s type, and the
    target has the Restrained condition until the spell ends. At the start of each of its turns, the Restrained
    target repeats the save. On a failed save, the target takes 4d8 damage of the spirit’s type. On a
    successful save, the target isn’t Restrained by the spirit. Using a Higher-Level Spell Slot. The damage
    increases by 1d8 for each spell slot level above 5.'
  sha256: 512788ce824724659d3404329e876dafb1dd32df269dd5dc427ae5e3f0785801
  status: needs_semantic_conversion
---

# Conjure Elemental

<!-- SRD page 118 -->

Level 5 Conjuration (Druid, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure a Large, intangible spirit from the Ele-\
mental Planes that appears in an unoccupied space\
within range. Choose the spirit’s element, which\
determines its damage type: air (Lightning), earth\
(Thunder), fire (Fire), or water (Cold). The spirit\
lasts for the duration.\
Whenever a creature you can see enters the spir-\
it’s space or starts its turn within 5 feet of the spirit,\
you can force that creature to make a Dexterity sav-\
ing throw if the spirit has no creature Restrained.\
On failed save, the target takes 8d8 damage of the\
spirit’s type, and the target has the Restrained\
condition until the spell ends. At the start of each\
of its turns, the Restrained target repeats the save.\
On a failed save, the target takes 4d8 damage of the\
spirit’s type. On a successful save, the target isn’t\
Restrained by the spirit.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]].
