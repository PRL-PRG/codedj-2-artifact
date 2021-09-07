# Model report for file:///tmp/top-repos-quality-repos-6vi0e0f1/learnvuejs.git HEAD 714a4570c202aa412e8a5dbe044aed6271cdb685

### Dump

```json
{'created_at': '2021-09-01 16:52:33',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.5 kB',
 'tags': [],
 'uuid': '056ef761-3ae0-4639-9a61-8b406abd7c12',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-6vi0e0f1/learnvuejs.git 714a4570c202aa412e8a5dbe044aed6271cdb685

# javascript
108 rules, avg.len. 7.3
## train
PPCR: 0.937471
### report
macro
{'f1-score': 0.6834871395414404,
 'precision': 0.684674240664666,
 'recall': 0.6829049758530258,
 'support': 14258}
micro
{'f1-score': 0.9178706690980503,
 'precision': 0.9178706690980503,
 'recall': 0.9178706690980503,
 'support': 14258}
weighted
{'f1-score': 0.9081532084777258,
 'precision': 0.8991602761151574,
 'recall': 0.9178706690980503,
 'support': 14258}
### report_full
macro
{'f1-score': 0.6559832854422318,
 'precision': 0.684674240664666,
 'recall': 0.6335295405616486,
 'support': 15209}
micro
{'f1-score': 0.8882478704992025,
 'precision': 0.9178706690980503,
 'recall': 0.8604773489381288,
 'support': 15209}
weighted
{'f1-score': 0.8720681301047477,
 'precision': 0.8883837465485731,
 'recall': 0.8604773489381288,
 'support': 15209}
## test
PPCR: 0.930955
### report
macro
{'f1-score': 0.673203319290903,
 'precision': 0.666318522914696,
 'recall': 0.6805416157799686,
 'support': 3627}
micro
{'f1-score': 0.9073614557485525,
 'precision': 0.9073614557485525,
 'recall': 0.9073614557485525,
 'support': 3627}
weighted
{'f1-score': 0.8954494097232818,
 'precision': 0.8840365227094451,
 'recall': 0.9073614557485525,
 'support': 3627}
### report_full
macro
{'f1-score': 0.6442978627585914,
 'precision': 0.666318522914696,
 'recall': 0.6276011899861391,
 'support': 3896}
micro
{'f1-score': 0.8749169214409145,
 'precision': 0.9073614557485525,
 'recall': 0.8447125256673511,
 'support': 3896}
weighted
{'f1-score': 0.8558387337636212,
 'precision': 0.8711548537804155,
 'recall': 0.8447125256673511,
 'support': 3896}
```

## javascript
### Summary
78 rules, avg.len. 7.1

| | |
|-|-|
|Min support|187|
|Max support|2101|
|Min confidence|0.9204081892967224|
|Max confidence|0.9994266033172607|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.981. Support: 340.` |
| 2 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.936. Support: 730.` |
| 3 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 872.` |
| 4 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 925.` |
| 5 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 458.` |
| 6 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.roles in {MAP} and not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.931. Support: 630.` |
| 7 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.922. Support: 402.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 257.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 447.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 345.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 195.` |
| 12 | `  -1.internal_type not in {Identifier}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.978. Support: 904.` |
| 13 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 896.` |
| 14 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.925. Support: 404.` |
| 15 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 620.` |
| 16 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.934. Support: 388.` |
| 17 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 262.` |
| 18 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 510.` |
| 19 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 332.` |
| 20 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.944. Support: 275.` |
| 21 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.982. Support: 368.` |
| 22 | `  -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 917.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.949. Support: 477.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION, STRING} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.955. Support: 342.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 224.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.947. Support: 292.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 454.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≥ 11<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 245.` |
| 30 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1778.` |
| 31 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.959. Support: 307.` |
| 32 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 242.` |
| 33 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.956. Support: 305.` |
| 34 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.942. Support: 712.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.942. Support: 458.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 413.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 343.` |
| 38 | `  -1.internal_type = CommentLine<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 202.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 2101.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 287.` |
| 42 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 414.` |
| 43 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.roles in {MAP} and not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 540.` |
| 44 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.946. Support: 383.` |
| 45 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 265.` |
| 46 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 211.` |
| 47 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 472.` |
| 48 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.reserved not in {,, const, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 317.` |
| 49 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.roles in {KEY} and not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 578.` |
| 50 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.923. Support: 368.` |
| 51 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 239.` |
| 52 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, KEY, LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 53 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, KEY, LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 447.` |
| 54 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION, KEY, LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 301.` |
| 55 | `  -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 922.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 885.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.945. Support: 392.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {MAP} and not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 586.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.947. Support: 369.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 253.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 462.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, LITERAL, MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 352.` |
| 63 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 187.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, EXPRESSION, LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.925. Support: 273.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 900.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.934. Support: 448.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {KEY} and not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 575.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.941. Support: 380.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 245.` |
| 70 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {:, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 206.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, KEY, LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 457.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, const, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, KEY, LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 345.` |
| 73 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 956.` |
| 74 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.947. Support: 292.` |
| 75 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IDENTIFIER} and not in {LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 252.` |
| 76 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {IDENTIFIER, LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 455.` |
| 77 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {IDENTIFIER, LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 317.` |
| 78 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {COMMENT, IDENTIFIER, LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 205.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.128205128205129, "max_conf": 0.9994266033172607, "max_support": 2101, "min_conf": 0.9204081892967224, "min_support": 187, "num_rules": 78}}
```
</details>
