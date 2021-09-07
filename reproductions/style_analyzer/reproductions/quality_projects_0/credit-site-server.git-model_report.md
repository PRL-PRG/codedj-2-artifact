# Model report for file:///tmp/top-repos-quality-repos-dghgzwi4/credit-site-server.git HEAD 36e265a6d4834ae9a8f251c72da47d3bd7a8b58b

### Dump

```json
{'created_at': '2021-09-01 19:56:44',
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
 'size': '17.4 kB',
 'tags': [],
 'uuid': 'b25036d3-32c3-4259-b5bc-5fec027082dc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dghgzwi4/credit-site-server.git 36e265a6d4834ae9a8f251c72da47d3bd7a8b58b

# javascript
62 rules, avg.len. 5.8
## train
PPCR: 0.893516
### report
macro
{'f1-score': 0.8696456909135029,
 'precision': 0.901767644796991,
 'recall': 0.8461026310131352,
 'support': 7745}
micro
{'f1-score': 0.9106520335700451,
 'precision': 0.9106520335700452,
 'recall': 0.9106520335700452,
 'support': 7745}
weighted
{'f1-score': 0.9083074803589566,
 'precision': 0.9104526024632064,
 'recall': 0.9106520335700452,
 'support': 7745}
### report_full
macro
{'f1-score': 0.7657293973768974,
 'precision': 0.901767644796991,
 'recall': 0.6840604381173315,
 'support': 8668}
micro
{'f1-score': 0.8594406872600987,
 'precision': 0.9106520335700452,
 'recall': 0.813682510383018,
 'support': 8668}
weighted
{'f1-score': 0.8503667186230772,
 'precision': 0.9078192105636914,
 'recall': 0.813682510383018,
 'support': 8668}
## test
PPCR: 0.894561
### report
macro
{'f1-score': 0.869575191296397,
 'precision': 0.9024072813219347,
 'recall': 0.8456865359879346,
 'support': 1875}
micro
{'f1-score': 0.9173333333333333,
 'precision': 0.9173333333333333,
 'recall': 0.9173333333333333,
 'support': 1875}
weighted
{'f1-score': 0.9148119842685185,
 'precision': 0.9159891081893418,
 'recall': 0.9173333333333333,
 'support': 1875}
### report_full
macro
{'f1-score': 0.7732005729999581,
 'precision': 0.9024072813219347,
 'recall': 0.6939097767354678,
 'support': 2096}
micro
{'f1-score': 0.8662805338705615,
 'precision': 0.9173333333333333,
 'recall': 0.8206106870229007,
 'support': 2096}
weighted
{'f1-score': 0.8591781990845798,
 'precision': 0.914780451523661,
 'recall': 0.8206106870229007,
 'support': 2096}
```

## javascript
### Summary
35 rules, avg.len. 5.8

| | |
|-|-|
|Min support|165|
|Max support|1748|
|Min confidence|0.9356846213340759|
|Max confidence|0.998516321182251|

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
| 1 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1572.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 197.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.976. Support: 188.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 293.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, {, }}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 179.` |
| 6 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1553.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.960. Support: 211.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.997. Support: 192.` |
| 9 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 217.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MAP} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 235.` |
| 11 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.976. Support: 187.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.997. Support: 183.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≥ 3<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 218.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 291.` |
| 15 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 206.` |
| 16 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 1748.` |
| 17 | `  -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 204.` |
| 18 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 192.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 337.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 305.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 540.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 317.` |
| 23 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.967. Support: 197.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 180.` |
| 25 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 283.` |
| 26 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {;, {, }}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 180.` |
| 27 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 180.` |
| 28 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 165.` |
| 29 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 234.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.956. Support: 192.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MAP} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 241.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 4<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 172.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, {, }}<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 181.` |
| 34 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.963. Support: 228.` |
| 35 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 215.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.828571428571428, "max_conf": 0.998516321182251, "max_support": 1748, "min_conf": 0.9356846213340759, "min_support": 165, "num_rules": 35}}
```
</details>
