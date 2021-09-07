# Model report for file:///tmp/top-repos-quality-repos-7ykqbg8c/til.git HEAD 50893560b6c8f3366d2d039ab9fb9ac5db678a85

### Dump

```json
{'created_at': '2021-09-01 23:11:09',
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
 'size': '19.8 kB',
 'tags': [],
 'uuid': '3dfe1cc7-4fd6-42cc-86dc-302aa0b4f642',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7ykqbg8c/til.git 50893560b6c8f3366d2d039ab9fb9ac5db678a85

# javascript
23 rules, avg.len. 9.5
## train
PPCR: 0.833480
### report
macro
{'f1-score': 0.32553884289599333,
 'precision': 0.33305287240243486,
 'recall': 0.3191872946271475,
 'support': 24576}
micro
{'f1-score': 0.9441324869791666,
 'precision': 0.9441324869791666,
 'recall': 0.9441324869791666,
 'support': 24576}
weighted
{'f1-score': 0.9398891632691125,
 'precision': 0.9360590413639182,
 'recall': 0.9441324869791666,
 'support': 24576}
### report_full
macro
{'f1-score': 0.2761751970177841,
 'precision': 0.33305287240243486,
 'recall': 0.26236714264993655,
 'support': 29486}
micro
{'f1-score': 0.8583848174318375,
 'precision': 0.9441324869791666,
 'recall': 0.7869158244590653,
 'support': 29486}
weighted
{'f1-score': 0.8026016477097618,
 'precision': 0.8592240859873359,
 'recall': 0.7869158244590653,
 'support': 29486}
## test
PPCR: 0.843774
### report
macro
{'f1-score': 0.32566062456543066,
 'precision': 0.3256813618951438,
 'recall': 0.32689609406095177,
 'support': 6519}
micro
{'f1-score': 0.936953520478601,
 'precision': 0.936953520478601,
 'recall': 0.936953520478601,
 'support': 6519}
weighted
{'f1-score': 0.9337642040352813,
 'precision': 0.9310531186786234,
 'recall': 0.936953520478601,
 'support': 6519}
### report_full
macro
{'f1-score': 0.2719544147350614,
 'precision': 0.3256813618951438,
 'recall': 0.2556225166047104,
 'support': 7726}
micro
{'f1-score': 0.8575640575640575,
 'precision': 0.936953520478601,
 'recall': 0.7905772715506083,
 'support': 7726}
weighted
{'f1-score': 0.8073006312341979,
 'precision': 0.8591451498606891,
 'recall': 0.7905772715506083,
 'support': 7726}
```

## javascript
### Summary
15 rules, avg.len. 9.7

| | |
|-|-|
|Min support|110|
|Max support|10082|
|Min confidence|0.9235893487930298|
|Max confidence|0.9986910820007324|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
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
| 1 | `  -1.internal_type not in {CommentLine}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 126.` |
| 2 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.995. Support: 110.` |
| 3 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.924. Support: 1276.` |
| 4 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.999. Support: 382.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 386.` |
| 7 | `  •••start_col ≤ 15<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.944. Support: 561.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 171.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.951. Support: 174.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 180.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 170.` |
| 12 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, return, {, }}<br>	∧ -2.label in {<newline>}<br>	∧ -2.length ≤ 20<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, return, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 147.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, return, {, }}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.length ≤ 20<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, return, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1022.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, return, {, }}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.length ≤ 20<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, return, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 461.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, return, {, }}<br>	∧ -2.length ≤ 20<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, return, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 10082.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.733333333333333, "max_conf": 0.9986910820007324, "max_support": 10082, "min_conf": 0.9235893487930298, "min_support": 110, "num_rules": 15}}
```
</details>
