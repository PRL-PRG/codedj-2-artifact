# Model report for file:///tmp/top-repos-quality-repos-anl6kymb/node-susanin.git HEAD 6f35e4da69815d6250e16655af8a32bcaded0ea7

### Dump

```json
{'created_at': '2021-09-02 08:23:10',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': '0d641dbc-73ca-4954-8964-e869df9e87dc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-anl6kymb/node-susanin.git 6f35e4da69815d6250e16655af8a32bcaded0ea7

# javascript
89 rules, avg.len. 5.0
## train
PPCR: 0.914848
### report
macro
{'f1-score': 0.6217138903192251,
 'precision': 0.640869918556147,
 'recall': 0.6089711736239363,
 'support': 7209}
micro
{'f1-score': 0.8963795255930087,
 'precision': 0.8963795255930087,
 'recall': 0.8963795255930087,
 'support': 7209}
weighted
{'f1-score': 0.8882413054337572,
 'precision': 0.88508479197146,
 'recall': 0.8963795255930087,
 'support': 7209}
### report_full
macro
{'f1-score': 0.5965733924242365,
 'precision': 0.640869918556147,
 'recall': 0.5693699616406892,
 'support': 7880}
micro
{'f1-score': 0.8565179932401087,
 'precision': 0.8963795255930087,
 'recall': 0.8200507614213198,
 'support': 7880}
weighted
{'f1-score': 0.8274524077141177,
 'precision': 0.844692938951544,
 'recall': 0.8200507614213198,
 'support': 7880}
## test
PPCR: 0.900165
### report
macro
{'f1-score': 0.6288731062790172,
 'precision': 0.6288917622685879,
 'recall': 0.6334883653299103,
 'support': 1641}
micro
{'f1-score': 0.8897014015843998,
 'precision': 0.8897014015843998,
 'recall': 0.8897014015843998,
 'support': 1641}
weighted
{'f1-score': 0.8827466373517224,
 'precision': 0.8829270836160138,
 'recall': 0.8897014015843998,
 'support': 1641}
### report_full
macro
{'f1-score': 0.6035923583711671,
 'precision': 0.6288917622685879,
 'recall': 0.5934100464442146,
 'support': 1823}
micro
{'f1-score': 0.8429561200923786,
 'precision': 0.8897014015843998,
 'recall': 0.8008776741634668,
 'support': 1823}
weighted
{'f1-score': 0.8099825416078803,
 'precision': 0.8317126465058287,
 'recall': 0.8008776741634668,
 'support': 1823}
```

## javascript
### Summary
47 rules, avg.len. 4.2

| | |
|-|-|
|Min support|139|
|Max support|1535|
|Min confidence|0.9242957830429077|
|Max confidence|0.9985875487327576|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.998. Support: 204.` |
| 2 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 231.` |
| 3 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 170.` |
| 4 | `  +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 239.` |
| 5 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 156.` |
| 6 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.995. Support: 330.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = '<br>Confidence: 0.998. Support: 242.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1494.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.999. Support: 354.` |
| 10 | `  -3.length ≥ 5<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 406.` |
| 11 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 1405.` |
| 12 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 255.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 219.` |
| 14 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 247.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 265.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 249.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.996. Support: 354.` |
| 18 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 202.` |
| 19 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 1523.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 237.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 254.` |
| 22 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 23 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 148.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 139.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION} and not in {ARGUMENT}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 292.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 4<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 158.` |
| 28 | `  +1.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 335.` |
| 29 | `  -4.diff_offset ≥ 5<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 284.` |
| 30 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 239.` |
| 31 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 227.` |
| 32 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 156.` |
| 33 | `  +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.994. Support: 387.` |
| 34 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.947. Support: 294.` |
| 35 | `  -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 232.` |
| 36 | `  -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 221.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1535.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.999. Support: 344.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 40 | `  -1.reserved = ,<br>	∧ +1.roles in {EXPRESSION, MAP}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 240.` |
| 41 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = '<br>Confidence: 0.998. Support: 252.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1520.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 344.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 45 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.998. Support: 229.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 203.` |
| 47 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 142.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.23404255319149, "max_conf": 0.9985875487327576, "max_support": 1535, "min_conf": 0.9242957830429077, "min_support": 139, "num_rules": 47}}
```
</details>
