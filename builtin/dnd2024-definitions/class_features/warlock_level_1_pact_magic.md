---
splat_schema: 1
splat_id: 184bfb4e-d9b3-57e5-8cc9-a7e99c76dbdf
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_1_pact_magic
splat_name: 'Level 1: Pact Magic'
splat_origin:
  key: dnd2024:reference/pages/071
splat_sources:
- title: SRD 5.2.1
  locator: pp. 71, 72
  url: https://www.dndbeyond.com/srd
---

# Level 1: Pact Magic

## Source text

<!-- SRD page 71 -->

Through occult ceremony, you have formed a pact\
with a mysterious entity to gain magical powers.\
The entity is a voice in the shadows—its identity\
unclear—but its boon to you is concrete: the ability\
to cast spells. See “Spells” for the rules on spell-\
casting. The information below details how you use\
those rules with Warlock spells, which appear in the\
Warlock spell list later in the class’s description.\
Cantrips. You know two Warlock cantrips of your\
choice. Eldritch Blast and Prestidigitation are rec-\
ommended. Whenever you gain a Warlock level, you\
can replace one of your cantrips from this feature\
with another Warlock cantrip of your choice.\
When you reach Warlock levels 4 and 10, you\
learn another Warlock cantrip of your choice, as\
shown in the Cantrips column of the Warlock Fea-\
tures table.\
Eldritch Prepared Spell Slot\
Invocations Cantrips Spells Slots Level\
1 2 2 1 1\
3 2 3 2 1\
3 2 4 2 2\
3 3 5 2 2\
5 3 6 2 3\
5 3 7 2 3\
6 3 8 2 4\
6 3 9 2 4\
7 3 10 2 5\
7 4 10 2 5\
7 4 11 3 5\
8 4 11 3 5\
8 4 12 3 5\
8 4 12 3 5\
9 4 13 3 5\
9 4 13 3 5\
9 4 14 4 5\
10 4 14 4 5\
10 4 15 4 5\
10 4 15 4 5\
Spell Slots. The Warlock Features table shows\
how many spell slots you have to cast your Warlock\
spells of levels 1–5. The table also shows the level\
of those slots, all of which are the same level. You\
regain all expended Pact Magic spell slots when you\
finish a Short or Long Rest.\
For example, when you’re a level 5 Warlock, you\
have two level 3 spell slots. To cast the level 1 spell\
Charm Person, you must spend one of those slots,\
and you cast it as a level 3 spell.\
Prepared Spells of Level 1+. You prepare the\
list of level 1+ spells that are available for you\
to cast with this feature. To start, choose two\
level 1 Warlock spells. Charm Person and Hex are\
recommended.\
The number of spells on your list increases as you\
gain Warlock levels, as shown in the Prepared Spells\
column of the Warlock Features table. Whenever\
that number increases, choose additional War-\
lock spells until the number of spells on your list\
matches the number in the table. The chosen spells\
must be of a level no higher than what’s shown in\
the table’s Slot Level column for your level. When\
you reach level 6, for example, you learn a new War-\
lock spell, which can be of levels 1–3.\
If another Warlock feature gives you spells that\
you always have prepared, those spells don’t count\

<!-- SRD page 72 -->

against the number of spells you can prepare with\
this feature, but those spells otherwise count as\
Warlock spells for you.\
Changing Your Prepared Spells. Whenever you\
gain a Warlock level, you can replace one spell on\
your list with another Warlock spell of an eligible\
level.\
Spellcasting Ability. Charisma is the spellcasting\
ability for your Warlock spells.\
Spellcasting Focus. You can use an Arcane Focus\
as a Spellcasting Focus for your Warlock spells.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warlock_level_1_pact_magic ` |
| ` implementation ` | ` dnd2024:class_features_warlock_level_1_pact_magic ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` spellcasting_ability ` | ` charisma ` |
| ` spellcasting_focus ` | ` arcane_focus ` |

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.charisma.modifier
- var: actor.proficiency_bonus
```

#### Spell attack bonus

<!-- splat:yaml path=splat_rule/parameters/spell_attack_bonus -->

```yaml
op: add
args:
- var: actor.proficiency_bonus
- var: actor.abilities.charisma.modifier
```

#### Slots

<!-- splat:yaml path=splat_rule/parameters/slots -->

```yaml
progression:
  key: dnd2024:tables/warlock_progression
recovery:
- rest.short_completed
- rest.long_completed
recover: all
pool: pact_magic
count:
  table:
    key: dnd2024:tables/warlock_progression
  row:
    var: actor.class_levels.warlock
  column: pact_slots
level:
  table:
    key: dnd2024:tables/warlock_progression
  row:
    var: actor.class_levels.warlock
  column: pact_slot_level
uniform_level: true
spell_level_range:
- 1
- 5
cast_at_slot_level: true
```

#### Prepared spells

<!-- splat:yaml path=splat_rule/parameters/prepared_spells -->

```yaml
initial_count: 2
initial_spell_level: 1
progression:
  table:
    key: dnd2024:tables/warlock_progression
  row:
    var: actor.class_levels.warlock
  column: prepared_spells
eligible_list: warlock
eligible_levels:
  minimum: 1
  maximum: pact_slot_level
on_count_increase: choose_until_count_met
replacement:
  event: character.level_gained
  class: warlock
  maximum: 1
always_prepared_from_other_class_features:
  counts_against_limit: false
  counts_as_class_spell: warlock
```

#### Cantrips

<!-- splat:yaml path=splat_rule/parameters/cantrips -->

```yaml
initial_count: 2
list: warlock
additional_levels:
- 4
- 10
additional_each: 1
replace:
  event: character.level_gained
  class: warlock
  maximum: 1
  same_list: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` minimum_level ` | ` 1 ` |
