# Model report for file:///tmp/top-repos-quality-repos-9_z3hcyn/sfchat.git HEAD 4422b4c2e172c238248b2d99ef2d82912710dbf9

### Dump

```json
{'created_at': '2021-09-02 03:21:10',
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
 'size': '18.6 kB',
 'tags': [],
 'uuid': '7c8d8b3f-f87b-435b-b5e0-38c422869980',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9_z3hcyn/sfchat.git 4422b4c2e172c238248b2d99ef2d82912710dbf9

# javascript
93 rules, avg.len. 6.4
## train
PPCR: 0.950168
### report
macro
{'f1-score': 0.6984456436201294,
 'precision': 0.782677486262834,
 'recall': 0.6490030073776569,
 'support': 14167}
micro
{'f1-score': 0.8979318133690972,
 'precision': 0.8979318133690972,
 'recall': 0.8979318133690972,
 'support': 14167}
weighted
{'f1-score': 0.8876807414587219,
 'precision': 0.8945149869496641,
 'recall': 0.8979318133690972,
 'support': 14167}
### report_full
macro
{'f1-score': 0.6576599148093132,
 'precision': 0.782677486262834,
 'recall': 0.5932693234191894,
 'support': 14910}
micro
{'f1-score': 0.8749871032087216,
 'precision': 0.8979318133690972,
 'recall': 0.8531857813547954,
 'support': 14910}
weighted
{'f1-score': 0.855597816170525,
 'precision': 0.8920261003693146,
 'recall': 0.8531857813547954,
 'support': 14910}
## test
PPCR: 0.958856
### report
macro
{'f1-score': 0.6935543489119516,
 'precision': 0.7897333540544927,
 'recall': 0.6450670899363654,
 'support': 2983}
micro
{'f1-score': 0.8773047267851156,
 'precision': 0.8773047267851156,
 'recall': 0.8773047267851156,
 'support': 2983}
weighted
{'f1-score': 0.8647120291060305,
 'precision': 0.878996601585262,
 'recall': 0.8773047267851156,
 'support': 2983}
### report_full
macro
{'f1-score': 0.6636640254526895,
 'precision': 0.7897333540544927,
 'recall': 0.6053736196835944,
 'support': 3111}
micro
{'f1-score': 0.8588775845093534,
 'precision': 0.8773047267851156,
 'recall': 0.8412086145933784,
 'support': 3111}
weighted
{'f1-score': 0.8388580952724572,
 'precision': 0.8791647871457487,
 'recall': 0.8412086145933784,
 'support': 3111}
```

## javascript
### Summary
45 rules, avg.len. 5.8

| | |
|-|-|
|Min support|170|
|Max support|7015|
|Min confidence|0.9208494424819946|
|Max confidence|0.9985822439193726|

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
                     'min_samples_leaf': 110,
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.969. Support: 595.` |
| 2 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.985. Support: 302.` |
| 3 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 180.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.978. Support: 292.` |
| 5 | `  -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 217.` |
| 6 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -5.length ≤ 17<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 232.` |
| 7 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -5.length ≤ 17<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 6356.` |
| 8 | `  -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.947. Support: 426.` |
| 9 | `  -1.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.963. Support: 558.` |
| 10 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.973. Support: 274.` |
| 11 | `  •••start_col ≥ 17<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -5.length ≤ 20<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 200.` |
| 12 | `  •••start_col ≥ 17<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -5.length ≤ 20<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 6188.` |
| 13 | `  -1.reserved not in {,}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.937. Support: 407.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {STRING}<br>⇒ y = '<br>Confidence: 0.953. Support: 566.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ␣<br>Confidence: 0.997. Support: 170.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.969. Support: 274.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ -5.diff_offset ≤ 58<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 312.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -5.diff_offset ≤ 58<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 7015.` |
| 19 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 5311.` |
| 20 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.950. Support: 351.` |
| 21 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALL}<br>⇒ y = '<br>Confidence: 0.959. Support: 181.` |
| 22 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 179.` |
| 23 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3174.` |
| 24 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 217.` |
| 25 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.978. Support: 207.` |
| 26 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 27 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 961.` |
| 28 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.952. Support: 591.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.953. Support: 395.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 244.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.966. Support: 279.` |
| 32 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.944. Support: 310.` |
| 33 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ␣<br>Confidence: 0.998. Support: 207.` |
| 34 | `  •••start_col ≥ 18<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -5.length ≤ 17<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {MAP} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 325.` |
| 35 | `  •••start_col ≥ 18<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -5.length ≤ 17<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {MAP, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 6019.` |
| 36 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 386.` |
| 37 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 6986.` |
| 38 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3228.` |
| 39 | `  -1.reserved = (<br>	∧ +1.internal_type = Identifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 479.` |
| 40 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.964. Support: 294.` |
| 41 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.921. Support: 259.` |
| 42 | `  -1.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.959. Support: 506.` |
| 43 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.931. Support: 312.` |
| 44 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 356.` |
| 45 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.822222222222222, "max_conf": 0.9985822439193726, "max_support": 7015, "min_conf": 0.9208494424819946, "min_support": 170, "num_rules": 45}}
```
</details>
