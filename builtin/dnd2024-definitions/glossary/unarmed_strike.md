---
splat_schema: 1
splat_id: 0a5955ef-690d-5d69-89d2-f2ee734745b3
splat_kind: rule
splat_key: dnd2024:glossary/unarmed_strike
splat_name: Unarmed Strike
splat_origin:
  key: dnd2024:reference/glossary/unarmed_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/unarmed_strike
  parameters:
    attack_type: melee
    weapon_used: false
    body_strike_examples:
    - punch
    - kick
    - headbutt
    - similar_forceful_blow
    target_range_feet: 5
    effect_choice:
      chooser: subject
      options:
      - damage
      - grapple
      - shove
    damage:
      attack_bonus:
        op: add
        args:
        - var: strength_modifier
        - var: proficiency_bonus
      hit_damage:
        type: bludgeoning
        amount:
          op: add
          args:
          - 1
          - var: strength_modifier
    grapple:
      target_maximum_size_categories_larger: 1
      free_hand_required: true
      save_ability_choice:
        chooser: target
        options:
        - strength
        - dexterity
      save_dc:
        op: add
        args:
        - 8
        - var: strength_modifier
        - var: proficiency_bonus
      escape_dc: same_as_save_dc
      failure_condition: grappled
    shove:
      target_maximum_size_categories_larger: 1
      save_ability_choice:
        chooser: target
        options:
        - strength
        - dexterity
      save_dc:
        op: add
        args:
        - 8
        - var: strength_modifier
        - var: proficiency_bonus
      failure_choice:
        chooser: subject
        options:
        - push_feet: 5
          direction: away_from_subject
        - condition: prone
  implementation: dnd2024:glossary_unarmed_strike
splat_contracts:
- id: dnd2024:glossary_unarmed_strike
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: participant
      required: true
    option:
      type: choice
      required: true
    target_save_ability:
      required: false
      type: choice
    shove_result:
      required: false
      type: choice
  outputs:
  - attack_or_save
  - damage
  - condition_changes
  - forced_movement
  events:
  - attack.pending
  - saving_throw.pending
  capabilities:
  - attacks
  - checks
  - grappling
  - movement
  - damage
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 521bac6a9260fe8f552663861155ddf47952db53d62131e48a1168703231ccaf
  source_path: builtin/srd-5.2.1/glossary/unarmed_strike.md
  pages:
  - 190
  engine_gaps: &id001
  - dnd2024:glossary_unarmed_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 69f546bce4a7f258965494b507b7fcd8e5318555e20eb6efb705d61c9f24e5b3
splat_clauses:
- key: source_rules
  text: '# Unarmed Strike Instead of using a weapon to make a melee attack, you can use a punch, kick,
    headbutt, or similar forceful blow. In game terms, this is an Unarmed Strike—a melee attack that involves
    you using your body to damage, grapple, or shove a target within 5 feet of you. Whenever you use your
    Unarmed Strike, choose one of the following options for its effect. Damage. You make an attack roll
    against the target. Your bonus to the roll equals your Strength modifier plus your Proficiency Bonus.
    On a hit, the target takes Bludgeoning damage equal to 1 plus your Strength modifier. Grapple. The
    target must succeed on a Strength or Dexterity saving throw (it chooses which), or it has the Grappled
    condition. The DC for the saving throw and any escape attempts equals 8 plus your Strength modifier
    and Proficiency Bonus. This grapple is possible only if the target is no more than one size larger
    than you and if you have a hand free to grab it. See also “Grappling.” Shove. The target must succeed
    on a Strength or Dexterity saving throw (it chooses which), or you either push it 5 feet away or cause
    it to have the Prone condition. The DC for the saving throw equals 8 plus your Strength modifier and
    Proficiency Bonus. This shove is possible only if the target is no more than one size larger than
    you.'
  sha256: c018b7513079b57068a1907b2588f2644b1547bd1e493d5ee97bd0edef56837a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Unarmed Strike

<!-- SRD page 190 -->

Instead of using a weapon to make a melee attack,\
you can use a punch, kick, headbutt, or similar\
forceful blow. In game terms, this is an Unarmed\
Strike—a melee attack that involves you using your\
body to damage, grapple, or shove a target within 5\
feet of you.\
Whenever you use your Unarmed Strike, choose\
one of the following options for its effect.\
Damage. You make an attack roll against the\
target. Your bonus to the roll equals your Strength\
modifier plus your Proficiency Bonus. On a hit, the\
target takes Bludgeoning damage equal to 1 plus\
your Strength modifier.\
Grapple. The target must succeed on a Strength\
or Dexterity saving throw (it chooses which), or it\
has the Grappled condition. The DC for the saving\
throw and any escape attempts equals 8 plus your\
Strength modifier and Proficiency Bonus. This grap-\
ple is possible only if the target is no more than one\
size larger than you and if you have a hand free to\
grab it. See also “Grappling.”\
Shove. The target must succeed on a Strength or\
Dexterity saving throw (it chooses which), or you ei-\
ther push it 5 feet away or cause it to have the Prone\
condition. The DC for the saving throw equals 8 plus\
your Strength modifier and Proficiency Bonus. This\
shove is possible only if the target is no more than\
one size larger than you.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].
