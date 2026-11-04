---
splat_schema: 2
splat_id: b218697c-baf0-53fe-bea1-725364371d82
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_a_background
splat_name: Choose a Background
splat_origin: dnd2024:reference/pages/019
splat_sources:
- SRD 5.2.1, pp. 19, 20, https://www.dndbeyond.com/srd
---

# Choose a Background

## Source text

<!-- SRD page 19 -->

Choose your character’s background, and write it on your character sheet. You can choose any of the backgrounds detailed in “Character Origins,” and your GM might offer additional backgrounds as options.

The background you choose influences step 3, when you determine your character’s ability scores. If you’re having trouble choosing, the Ability Scores and Backgrounds table shows which backgrounds benefit which ability scores. Look for your class’s primary ability there.

**Ability Scores and Backgrounds**

| Ability | Background |
| --- | --- |
| Strength | Soldier |
| Dexterity | Soldier |
| Constitution | Soldier |
| Intelligence | Acolyte |

<!-- SRD page 20 -->

| Ability | Background |
| --- | --- |
| Wisdom | Acolyte |
| Charisma | Acolyte |

Record Your Feat. A background gives you a feat, which grants your character particular capabilities. Feats are detailed in “Feats.” Write the feat on your character sheet.

Note Proficiencies. Your background gives proficiency in two skills and with one tool. Record this information on your character sheet.

Your class also gives proficiencies. Check your class description in “Classes” and note the proficiencies on your character sheet.

The features table in your class description shows your Proficiency Bonus (described in “Playing the Game”), which is +2 for a level 1 character. Note this number on your character sheet. You’ll fill in other numbers connected to these proficiencies in step 5.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_choose_a_background ` |
| ` implementation ` | ` dnd2024:rules_create_your_character_choose_a_background ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class_proficiencies_also_granted ` | ` true ` |
| ` level_1_proficiency_bonus ` | ` 2 ` |

#### Background choice

<!-- splat:yaml path=splat_rule/parameters/background_choice -->

```yaml
count: 1
options:
- key: dnd2024:backgrounds/acolyte
- key: dnd2024:backgrounds/soldier
additional_options_authority: gm
```

#### Grants

<!-- splat:values path=splat_rule/parameters/grants -->

| Field | Value |
| --- | --- |
| ` feat ` | ` background_feat ` |
| ` skill_proficiencies ` | ` 2 ` |
| ` tool_proficiencies ` | ` 1 ` |

#### Ability background examples

<!-- splat:values path=splat_rule/parameters/ability_background_examples -->

| Field | Value |
| --- | --- |
| ` strength ` | ` soldier ` |
| ` dexterity ` | ` soldier ` |
| ` constitution ` | ` soldier ` |
| ` intelligence ` | ` acolyte ` |
| ` wisdom ` | ` acolyte ` |
| ` charisma ` | ` acolyte ` |
