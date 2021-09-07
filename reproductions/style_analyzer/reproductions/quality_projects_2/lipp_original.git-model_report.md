# Model report for file:///tmp/top-repos-quality-repos-mi28wn6_/lipp_original.git HEAD 0a004465cb7c5a7ab028ee2f71aa00f231a20f13

### Dump

```json
{'created_at': '2021-09-02 04:31:44',
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
 'size': '16.3 kB',
 'tags': [],
 'uuid': '19ad151e-e834-4082-8ef2-177d68913b28',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mi28wn6_/lipp_original.git 0a004465cb7c5a7ab028ee2f71aa00f231a20f13

# javascript
19 rules, avg.len. 6.1
## train
PPCR: 0.961149
### report
macro
{'f1-score': 0.9783891319950572,
 'precision': 0.977157433457091,
 'recall': 0.97995621470332,
 'support': 36466}
micro
{'f1-score': 0.9839302363845774,
 'precision': 0.9839302363845774,
 'recall': 0.9839302363845774,
 'support': 36466}
weighted
{'f1-score': 0.9839316698059903,
 'precision': 0.984096274286361,
 'recall': 0.9839302363845774,
 'support': 36466}
### report_full
macro
{'f1-score': 0.8968114765005105,
 'precision': 0.977157433457091,
 'recall': 0.861311227706038,
 'support': 37940}
micro
{'f1-score': 0.9644383517458269,
 'precision': 0.9839302363845774,
 'recall': 0.9457037427517132,
 'support': 37940}
weighted
{'f1-score': 0.9618549897987103,
 'precision': 0.9832582176751501,
 'recall': 0.9457037427517132,
 'support': 37940}
## test
PPCR: 0.993823
### report
macro
{'f1-score': 0.9767926964836935,
 'precision': 0.9622623363714459,
 'recall': 0.9964491277665269,
 'support': 8849}
micro
{'f1-score': 0.9970618148943383,
 'precision': 0.9970618148943383,
 'recall': 0.9970618148943383,
 'support': 8849}
weighted
{'f1-score': 0.9970691384934436,
 'precision': 0.9971150934403238,
 'recall': 0.9970618148943383,
 'support': 8849}
### report_full
macro
{'f1-score': 0.9557658755436895,
 'precision': 0.9622623363714459,
 'recall': 0.9495469038929869,
 'support': 8904}
micro
{'f1-score': 0.9939728496592125,
 'precision': 0.9970618148943383,
 'recall': 0.9909029649595688,
 'support': 8904}
weighted
{'f1-score': 0.9939503747531866,
 'precision': 0.9970672384138365,
 'recall': 0.9909029649595688,
 'support': 8904}
```

## javascript
### Summary
17 rules, avg.len. 5.7

| | |
|-|-|
|Min support|114|
|Max support|7789|
|Min confidence|0.9251968264579773|
|Max confidence|0.9998738765716553|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 1.000. Support: 3965.` |
| 2 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 3028.` |
| 3 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 3005.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 464.` |
| 5 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>⇒ y = '<br>Confidence: 1.000. Support: 3610.` |
| 6 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 127.` |
| 7 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 920.` |
| 8 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 958.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1116.` |
| 10 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 447.` |
| 11 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.996. Support: 139.` |
| 12 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 520.` |
| 13 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 165.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 1748.` |
| 15 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.996. Support: 366.` |
| 16 | `  -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.996. Support: 114.` |
| 17 | `  -1.reserved not in {if}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 7789.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.705882352941177, "max_conf": 0.9998738765716553, "max_support": 7789, "min_conf": 0.9251968264579773, "min_support": 114, "num_rules": 17}}
```
</details>
