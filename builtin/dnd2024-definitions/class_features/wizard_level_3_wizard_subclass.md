---
splat_schema: 1
splat_id: e19da891-98a2-5713-9d7c-755d273232d5
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_3_wizard_subclass
splat_name: 'Level 3: Wizard Subclass'
splat_origin:
  key: dnd2024:reference/pages/078
splat_sources:
- title: SRD 5.2.1
  locator: pp. 78, 79
  url: https://www.dndbeyond.com/srd
---

# Level 3: Wizard Subclass

## Source text

<!-- SRD page 78 -->

You gain a Wizard subclass of your choice. The\
Evoker subclass is detailed after this class’s de-\
scription. A subclass is a specialization that grants\
you features at certain Wizard levels. For the rest\
ExpandinG and rEplacinG a spEllBook\
The spells you add to your spellbook as you gain\
levels reflect your ongoing magical research, but you\
might find other spells during your adventures that\
you can add to the book. You could discover a Wiz-\
ard spell on a Spell Scroll, for example, and then copy\
it into your spellbook.\
Copying a Spell into the Book. When you find\
a level 1+ Wizard spell, you can copy it into your\
spellbook if it’s of a level you can prepare and if you\
have time to copy it. For each level of the spell, the\
transcription takes 2 hours and costs 50 GP. After-\
ward you can prepare the spell like the other spells\
in your spellbook.\
Copying the Book. You can copy a spell from\
your spellbook into another book. This is like copy-\
ing a new spell into your spellbook but faster, since\
you already know how to cast the spell. You need\
spend only 1 hour and 10 GP for each level of the\
copied spell.\
If you lose your spellbook, you can use the same\
procedure to transcribe the Wizard spells that you\
have prepared into a new spellbook. Filling out the\
remainder of the new book requires you to find new\
spells to do so. For this reason, many wizards keep a\
backup spellbook.\

<!-- SRD page 79 -->

of your career, you gain each of your subclass’s fea-\
tures that are of your Wizard level or lower.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_3_wizard_subclass ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_3_wizard_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: wizard
srd_options:
- key: dnd2024:subclasses/wizard_wizard_subclass_evoker
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |

#### Spellbook copying sidebar

<!-- splat:yaml path=splat_rule/parameters/spellbook_copying_sidebar -->

```yaml
copy_found_spell:
  list: wizard
  minimum_spell_level: 1
  must_be_level_can_prepare: true
  hours_per_spell_level: 2
  gp_per_spell_level: 50
  result: add_to_spellbook
copy_own_book_spell:
  hours_per_spell_level: 1
  gp_per_spell_level: 10
  result: add_to_destination_spellbook
replace_lost_book:
  eligible_spells: currently_prepared_wizard_spells
  hours_per_spell_level: 1
  gp_per_spell_level: 10
  other_spells_require_rediscovery: true
```
