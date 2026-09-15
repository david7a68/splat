---
splat_schema: 1
splat_id: 31cc69e3-da6c-59fe-8c35-b90997a3252d
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_2_wild_shape
splat_name: 'Level 2: Wild Shape'
splat_origin:
  key: dnd2024:reference/pages/042
splat_sources:
- title: SRD 5.2.1
  locator: pp. 42, 43
  url: https://www.dndbeyond.com/srd
---

# Level 2: Wild Shape

## Source text

<!-- SRD page 42 -->

The power of nature allows you to assume the form\
of an animal. As a Bonus Action, you shape-shift into\
a Beast form that you have learned for this feature\
(see “Known Forms” below). You stay in that form\
for a number of hours equal to half your Druid level\
or until you use Wild Shape again, have the Incapac-\
itated condition, or die. You can also leave the form\
early as a Bonus Action.\
Number of Uses. You can use Wild Shape twice.\
You regain one expended use when you finish a\
Short Rest, and you regain all expended uses when\
you finish a Long Rest.\
You gain additional uses when you reach certain\
Druid levels, as shown in the Wild Shape column of\
the Druid Features table.\
Known Forms. You know four Beast forms for this\
feature, chosen from among Beast stat blocks that\
have a maximum Challenge Rating of 1/4 and that\
lack a Fly Speed (see “Animals” in “Monsters” for\
stat block options). The Rat, Riding Horse, Spider,\
and Wolf are recommended. Whenever you finish a\
Long Rest, you can replace one of your known forms\
with another eligible form.\
When you reach certain Druid levels, your number\
of known forms and the maximum Challenge Rating\
for those forms increases, as shown in the Beast\
Shapes table. In addition, starting at level 8, you can\
adopt a form that has a Fly Speed.\
When choosing known forms, you may look in\
other sources for eligible Beasts if the Game Master\
permits you to do so.\
Beast Shapes\
Druid Known Max Fly\
Level Forms CR Speed\
2 4 1/4 No\
4 6 1/2 No\
8 8 1 Yes\

<!-- SRD page 43 -->

Rules While Shape-Shifted. While in a form, you\
retain your personality, memories, and ability to\
speak, and the following rules apply:\
Temporary Hit Points. When you assume a Wild\
Shape form, you gain a number of Temporary Hit\
Points equal to your Druid level.\
Game Statistics. Your game statistics are replaced\
by the Beast’s stat block, but you retain your crea-\
ture type; Hit Points; Hit Point Dice; Intelligence,\
Wisdom, and Charisma scores; class features;\
languages; and feats. You also retain your skill\
and saving throw proficiencies and use your Pro-\
ficiency Bonus for them, in addition to gaining the\
proficiencies of the creature. If a skill or saving\
throw modifier in the Beast’s stat block is higher\
than yours, use the one in the stat block.\
No Spellcasting. You can’t cast spells, but shape-\
shifting doesn’t break your Concentration or oth-\
erwise interfere with a spell you’ve already cast.\
Objects. Your ability to handle objects is deter-\
mined by the form’s limbs rather than your own.\
In addition, you choose whether your equipment\
falls in your space, merges into your new form, or\
is worn by it. Worn equipment functions as nor-\
mal, but the GM decides whether it’s practical for\
the new form to wear a piece of equipment based\
on the creature’s size and shape. Your equipment\
doesn’t change size or shape to match the new\
form, and any equipment that the new form can’t\
wear must either fall to the ground or merge with\
the form. Equipment that merges with the form\
has no effect while you’re in that form.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_2_wild_shape ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_2_wild_shape ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` cost ` | ` 1 ` |
| ` ends_on ` | ` ["wild_shape.used_again", "condition.incapacitated", "death"] ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: wild_shape
initial_maximum: 2
maximum_by_class_level:
  '2': 2
  '6': 3
  '17': 4
recover:
- event: rest.short_rest_completed
  amount: 1
- event: rest.long_rest_completed
  amount: all_expended
```

#### Form choice

<!-- splat:yaml path=splat_rule/parameters/form_choice -->

```yaml
from: known_beast_forms
known_count_by_druid_level:
  '2': 4
  '4': 6
  '8': 8
maximum_challenge_rating_by_druid_level:
  '2': 0.25
  '4': 0.5
  '8': 1
fly_speed_allowed_minimum_druid_level: 8
replace:
  event: rest.long_rest_completed
  maximum: 1
  must_remain_eligible: true
other_source_beasts:
  choice_authority: game_master
  type: allow_source
  required: true
```

#### Duration

<!-- splat:yaml path=splat_rule/parameters/duration -->

```yaml
hours:
  op: divide
  args:
  - stat: classes.druid.level
  - 2
```

#### End voluntarily

<!-- splat:values path=splat_rule/parameters/end_voluntarily -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |

#### Temporary hit points

<!-- splat:values path=splat_rule/parameters/temporary_hit_points -->

| Field | Value |
| --- | --- |
| ` stat ` | ` classes.druid.level ` |

#### Statistics

<!-- splat:yaml path=splat_rule/parameters/statistics -->

```yaml
replace_with: beast_stat_block
retain:
- personality
- memories
- speech
- creature_type
- hit_points
- hit_point_dice
- intelligence
- wisdom
- charisma
- class_features
- languages
- feats
proficiencies:
  retain:
  - skills
  - saving_throws
  use_own_proficiency_bonus: true
  gain_beast_proficiencies: true
  modifier_rule: higher_of_own_and_beast
```

#### Spellcasting

<!-- splat:values path=splat_rule/parameters/spellcasting -->

| Field | Value |
| --- | --- |
| ` can_cast ` | ` false ` |
| ` concentration_preserved ` | ` true ` |
| ` existing_spells_unaffected ` | ` true ` |

#### Objects

<!-- splat:yaml path=splat_rule/parameters/objects -->

```yaml
handling_depends_on: form_limbs
equipment_choice_per_item:
- drop_in_space
- merge
- wear
wearing_eligibility:
  authority: game_master
  decision_type: boolean
  criteria:
  - size
  - shape
equipment_resizes: false
unwearable_choices:
- drop_in_space
- merge
merged_equipment_effects_active: false
worn_equipment_effects_active: true
```
