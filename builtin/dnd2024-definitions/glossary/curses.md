---
splat_schema: 1
splat_id: ac741708-4e9f-5266-a8d4-3eb4c79342f2
splat_kind: rule
splat_key: dnd2024:glossary/curses
splat_name: Curses
splat_origin:
  key: dnd2024:reference/glossary/curses
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/curses
  parameters:
    target_types:
    - creature
    - object
    behavior: granting_effect_defined
    removal_sources:
    - key: dnd2024:spells/remove_curse
    - key: dnd2024:spells/greater_restoration
    other_removal_requires_explicit_curse_ending: true
  implementation: dnd2024:glossary_curses
splat_contracts:
- id: dnd2024:glossary_curses
  inputs:
    subject:
      type: object
      required: true
    curse:
      type: effect
      required: true
    removal_effect:
      required: false
      type: effect
  outputs:
  - curse_state
  events:
  - effect.applied
  - effect.removal_pending
  capabilities:
  - effects
  - curses
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 045265d77039c9d5bf75bafc51b4608dae957ac389d3cd7642e488a81f313a3e
  source_path: builtin/srd-5.2.1/glossary/curses.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_curses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 289856c927f1496345694e9949bbf2219c7c56bf80ab25d83905f2109dc5e7e1
splat_clauses:
- key: source_rules
  text: '# Curses Some game effects curse a creature or an object. The effect that confers a curse defines
    what the curse does. Curses can be removed by the Remove Curse and Greater Restoration spells or other
    magic that explicitly ends curses.'
  sha256: 43e58997811c5ef5dced234f518e16128d796edafa04ddc7a128e485a2d900f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Curses

<!-- SRD page 179 -->

Some game effects curse a creature or an object.\
The effect that confers a curse defines what the\
curse does. Curses can be removed by the Remove\
Curse and Greater Restoration spells or other magic\
that explicitly ends curses.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
