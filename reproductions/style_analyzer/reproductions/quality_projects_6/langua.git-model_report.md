# Model report for file:///tmp/top-repos-quality-repos-hq78x1nq/langua.git HEAD e46c690b3eec9f92e434060f6948d1759592140b

### Dump

```json
{'created_at': '2021-09-01 23:51:50',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.7 kB',
 'tags': [],
 'uuid': '0e921ca3-4468-4ace-a46f-676c073f4c5f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-hq78x1nq/langua.git e46c690b3eec9f92e434060f6948d1759592140b

# javascript
129 rules, avg.len. 10.8
## train
PPCR: 0.974235
### report
macro
{'f1-score': 0.8368215479191444,
 'precision': 0.9028202424545897,
 'recall': 0.7940943819294409,
 'support': 29115}
micro
{'f1-score': 0.9199725227545938,
 'precision': 0.9199725227545938,
 'recall': 0.9199725227545938,
 'support': 29115}
weighted
{'f1-score': 0.9176991675519343,
 'precision': 0.9207960942996984,
 'recall': 0.9199725227545938,
 'support': 29115}
### report_full
macro
{'f1-score': 0.794952529906558,
 'precision': 0.9028202424545897,
 'recall': 0.7380457788003911,
 'support': 29885}
micro
{'f1-score': 0.9079661016949153,
 'precision': 0.9199725227545938,
 'recall': 0.8962690312865986,
 'support': 29885}
weighted
{'f1-score': 0.9020890920144717,
 'precision': 0.9197570844184674,
 'recall': 0.8962690312865986,
 'support': 29885}
## test
PPCR: 0.977325
### report
macro
{'f1-score': 0.8496969666795201,
 'precision': 0.9093815233852736,
 'recall': 0.8103736390709457,
 'support': 6336}
micro
{'f1-score': 0.930239898989899,
 'precision': 0.930239898989899,
 'recall': 0.930239898989899,
 'support': 6336}
weighted
{'f1-score': 0.9278968424635949,
 'precision': 0.9304860554835825,
 'recall': 0.930239898989899,
 'support': 6336}
### report_full
macro
{'f1-score': 0.8089185011263244,
 'precision': 0.9093815233852736,
 'recall': 0.7484340975429424,
 'support': 6483}
micro
{'f1-score': 0.9195725095561276,
 'precision': 0.930239898989899,
 'recall': 0.9091469998457504,
 'support': 6483}
weighted
{'f1-score': 0.9148377447722694,
 'precision': 0.9296610097376407,
 'recall': 0.9091469998457504,
 'support': 6483}
```

## javascript
### Summary
64 rules, avg.len. 9.9

| | |
|-|-|
|Min support|135|
|Max support|4795|
|Min confidence|0.9213147163391113|
|Max confidence|0.9994813203811646|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 964.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 313.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 226.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3821.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 4709.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 325.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 2374.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.949. Support: 484.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 391.` |
| 10 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 997.` |
| 11 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.internal_type not in {MemberExpression, Program}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 229.` |
| 12 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression, Program}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 13 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression, Program}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 322.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 948.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 331.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 258.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 140.` |
| 18 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 272.` |
| 19 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 1786.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.999. Support: 959.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.947. Support: 465.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 243.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 165.` |
| 24 | `  •••start_col ≥ 7<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 148.` |
| 25 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 304.` |
| 26 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 1791.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3912.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 4693.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 322.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 2355.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.946. Support: 434.` |
| 32 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 928.` |
| 33 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 228.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.length ≥ 2<br>	∧ -3.diff_col ≥ 5<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 185.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 1397.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label in {<space>}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 157.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3883.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 4795.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 2321.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.948. Support: 471.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 211.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 164.` |
| 43 | `  •••start_col ≥ 6<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.length ≤ 16<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 159.` |
| 44 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.roles in {SCOPE} and not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 302.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 1637.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 338.` |
| 47 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression, Program}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 201.` |
| 48 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 1151.` |
| 49 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.length ≥ 2<br>	∧ -3.diff_col ≥ 5<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 194.` |
| 50 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 1448.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 225.` |
| 52 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 166.` |
| 53 | `  •••start_col ≤ 40<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {IMPORT}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, INITIALIZATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 1424.` |
| 54 | `  •••start_col ≤ 40<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.roles not in {IMPORT}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 16<br>	∧ ^1.roles in {BINARY} and not in {DECLARATION, INCOMPLETE, INITIALIZATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 185.` |
| 55 | `  •••start_col ≤ 40<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.roles not in {IMPORT}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, INITIALIZATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 430.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.length ≥ 2<br>	∧ -3.diff_col ≥ 5<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 180.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1348.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.921. Support: 502.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.947. Support: 441.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 994.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 220.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 135.` |
| 63 | `  •••start_col ≤ 26<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1021.` |
| 64 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 17<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 240.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.890625, "max_conf": 0.9994813203811646, "max_support": 4795, "min_conf": 0.9213147163391113, "min_support": 135, "num_rules": 64}}
```
</details>
