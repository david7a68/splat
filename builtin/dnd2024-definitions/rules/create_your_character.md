---
splat_schema: 1
splat_id: 9b5a401d-e630-5918-81ee-c6cc7d99328d
splat_kind: rule
splat_key: dnd2024:rules/create_your_character
splat_name: Create Your Character
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character
  parameters:
    sequence:
    - choose_class
    - choose_background
    - choose_species
    - choose_languages
    - assign_six_ability_scores
    - choose_alignment
    - fill_character_details
    class_count_at_creation: 1
    origin_components:
    - background
    - species
  implementation: dnd2024:rules_create_your_character
splat_contracts:
- id: dnd2024:rules_create_your_character
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - creation_steps
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 19
  engine_gaps: &id001
  - dnd2024:rules_create_your_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 0d431cbc6e85709c43535fa02e13669b73df6d2580d9e62792c5d3166ce9e862
  parent_name: Create Your Character
  line_span:
    first: 1328
    count: 18
splat_clauses:
- key: source_rules
  text: '# Create Your Character Here are the steps to make a character; each step is explored in detail:
    1: Choose a Class. Every adventurer is a member of a class. A class broadly describes a character’s
    vocation, special talents, and favored tactics. 2: Determine Origin. A character’s origin includes
    two elements: background and species. How did the character spend the years leading up to a life of
    adventure? Who are the character’s ancestors? You can also choose your character’s languages. 3: Determine
    Ability Scores. Much of what your character does in the game depends on the character’s six abilities.
    4: Choose an Alignment. Alignment is a shorthand for your character’s moral compass. 5: Fill in Details.
    Using the choices you’ve made, fill in the remaining details on your character sheet.'
  sha256: ce600864a83139487a483bfc531b51c1558b17e49f85dd746759355339e1d9b4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Create Your Character

<!-- SRD page 19 -->

Here are the steps to make a character; each step is\
explored in detail:\
1:  Choose a Class. Every adventurer is a member\
of a class. A class broadly describes a character’s\
vocation, special talents, and favored tactics.\
2:  Determine  Origin. A character’s origin includes\
two elements: background and species. How did\
the character spend the years leading up to a life\
of adventure? Who are the character’s ancestors?\
You can also choose your character’s languages.\
3: Determine Ability Scores. Much of what your\
character does in the game depends on the char-\
acter’s six abilities.\
4:  Choose an Alignment. Alignment is a shorthand\
for your character’s moral compass.\
5: Fill in Details. Using the choices you’ve made, fill\
in the remaining details on your character sheet.\
