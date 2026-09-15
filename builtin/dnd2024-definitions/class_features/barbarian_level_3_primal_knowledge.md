---
splat_schema: 1
splat_id: e3f958a2-c2be-5f89-8deb-0939ae637dd2
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_3_primal_knowledge
splat_name: 'Level 3: Primal Knowledge'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
---

# Level 3: Primal Knowledge

## Source text

<!-- SRD page 29 -->

You gain proficiency in another skill of your choice\
from the skill list available to Barbarians at level 1.\
In addition, while your Rage is active, you can\
channel primal power when you attempt certain\
tasks; whenever you make an ability check using\
one of the following skills, you can make it as a\
Strength check even if it normally uses a differ-\
ent ability: Acrobatics, Intimidation, Perception,\
Stealth, or Survival. When you use this ability, your\
Strength represents primal power coursing through\
you, honing your agility, bearing, and senses.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_3_primal_knowledge ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_3_primal_knowledge ` |

### Parameters

#### Skill proficiency

<!-- splat:values path=splat_rule/parameters/skill_proficiency -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` choices ` | ` ["animal_handling", "athletics", "intimidation", "nature", "perception", "survival"] ` |

#### Ability substitution

<!-- splat:yaml path=splat_rule/parameters/ability_substitution -->

```yaml
requires:
  rage_active: true
skills:
- acrobatics
- intimidation
- perception
- stealth
- survival
ability: strength
optional: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 3 ` |
