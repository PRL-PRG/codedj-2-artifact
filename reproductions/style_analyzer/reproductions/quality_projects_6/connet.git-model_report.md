# Model report for file:///tmp/top-repos-quality-repos-oeexcjg8/connet.git HEAD 7df0655579d699f9cf686acb28b953bad3b4b2bc

### Dump

```json
{'created_at': '2021-09-02 06:10:03',
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
 'size': '16.3 kB',
 'tags': [],
 'uuid': '33d47138-d283-43e7-8750-bfa74dcf0325',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-oeexcjg8/connet.git 7df0655579d699f9cf686acb28b953bad3b4b2bc

# javascript
18 rules, avg.len. 6.7
## train
PPCR: 0.855375
### report
macro
{'f1-score': 0.7421412548173326,
 'precision': 0.8140068464100922,
 'recall': 0.7001193573604493,
 'support': 16590}
micro
{'f1-score': 0.9373116335141651,
 'precision': 0.9373116335141651,
 'recall': 0.9373116335141651,
 'support': 16590}
weighted
{'f1-score': 0.9324174297337858,
 'precision': 0.9350774782104136,
 'recall': 0.9373116335141651,
 'support': 16590}
### report_full
macro
{'f1-score': 0.6400154331340021,
 'precision': 0.8140068464100922,
 'recall': 0.5583465140967331,
 'support': 19395}
micro
{'f1-score': 0.8642489926358204,
 'precision': 0.9373116335141651,
 'recall': 0.8017530291312194,
 'support': 19395}
weighted
{'f1-score': 0.845724008493815,
 'precision': 0.9278600932396908,
 'recall': 0.8017530291312194,
 'support': 19395}
## test
PPCR: 0.847057
### report
macro
{'f1-score': 0.7230699595206846,
 'precision': 0.7822027493817006,
 'recall': 0.6884182255618939,
 'support': 4835}
micro
{'f1-score': 0.9265770423991727,
 'precision': 0.9265770423991727,
 'recall': 0.9265770423991727,
 'support': 4835}
weighted
{'f1-score': 0.9203320692245212,
 'precision': 0.9216043261509501,
 'recall': 0.9265770423991727,
 'support': 4835}
### report_full
macro
{'f1-score': 0.6277574707571115,
 'precision': 0.7822027493817006,
 'recall': 0.5535718888494775,
 'support': 5708}
micro
{'f1-score': 0.8498529830219103,
 'precision': 0.9265770423991727,
 'recall': 0.7848633496846531,
 'support': 5708}
weighted
{'f1-score': 0.8300651033506855,
 'precision': 0.913460819010959,
 'recall': 0.7848633496846531,
 'support': 5708}
```

## javascript
### Summary
13 rules, avg.len. 7.1

| | |
|-|-|
|Min support|93|
|Max support|8318|
|Min confidence|0.9288891553878784|
|Max confidence|0.9994246363639832|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 869.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.978. Support: 649.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.996. Support: 134.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 477.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.978. Support: 114.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.978. Support: 161.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 374.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.962. Support: 119.` |
| 10 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, import}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.internal_type = CommentLine<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 93.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 122.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, import}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 147.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, import}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 8318.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.076923076923077, "max_conf": 0.9994246363639832, "max_support": 8318, "min_conf": 0.9288891553878784, "min_support": 93, "num_rules": 13}}
```
</details>
