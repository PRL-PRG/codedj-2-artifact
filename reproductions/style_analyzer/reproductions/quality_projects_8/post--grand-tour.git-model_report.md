# Model report for file:///tmp/top-repos-quality-repos-glf4lgz6/post--grand-tour.git HEAD 2859b7e14191360007d7bfbe0250332b51c74f6e

### Dump

```json
{'created_at': '2021-09-01 00:22:04',
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
 'size': '24.2 kB',
 'tags': [],
 'uuid': '0b49bbc1-17a7-47c9-bddb-2932ef1f93d5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-glf4lgz6/post--grand-tour.git 2859b7e14191360007d7bfbe0250332b51c74f6e

# javascript
51 rules, avg.len. 8.3
## train
PPCR: 0.897637
### report
macro
{'f1-score': 0.3945738896893068,
 'precision': 0.3936129626184755,
 'recall': 0.3961525842583774,
 'support': 116481}
micro
{'f1-score': 0.9590233600329667,
 'precision': 0.9590233600329667,
 'recall': 0.9590233600329667,
 'support': 116481}
weighted
{'f1-score': 0.9540281871518278,
 'precision': 0.9493060121044326,
 'recall': 0.9590233600329667,
 'support': 116481}
### report_full
macro
{'f1-score': 0.3613003108640717,
 'precision': 0.3936129626184755,
 'recall': 0.3388840404584879,
 'support': 129764}
micro
{'f1-score': 0.9072915186095148,
 'precision': 0.9590233600329667,
 'recall': 0.8608550907801855,
 'support': 129764}
weighted
{'f1-score': 0.8919531892605828,
 'precision': 0.9319851739683547,
 'recall': 0.8608550907801855,
 'support': 129764}
## test
PPCR: 0.872759
### report
macro
{'f1-score': 0.382978183758909,
 'precision': 0.3766288734966053,
 'recall': 0.39217730416522817,
 'support': 26188}
micro
{'f1-score': 0.9327936459447075,
 'precision': 0.9327936459447075,
 'recall': 0.9327936459447075,
 'support': 26188}
weighted
{'f1-score': 0.9239637234256537,
 'precision': 0.9169193976168394,
 'recall': 0.9327936459447075,
 'support': 26188}
### report_full
macro
{'f1-score': 0.34589003835176513,
 'precision': 0.3766288734966053,
 'recall': 0.32972949653851813,
 'support': 30006}
micro
{'f1-score': 0.869416663700751,
 'precision': 0.9327936459447075,
 'recall': 0.8141038458974872,
 'support': 30006}
weighted
{'f1-score': 0.8472923267739176,
 'precision': 0.8971684619693651,
 'recall': 0.8141038458974872,
 'support': 30006}
```

## javascript
### Summary
34 rules, avg.len. 8.4

| | |
|-|-|
|Min support|91|
|Max support|27686|
|Min confidence|0.9226804375648499|
|Max confidence|0.9981883764266968|

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
| 1 | `  -1.reserved = )<br>	∧ -2.internal_type = ThisExpression<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.930. Support: 122.` |
| 2 | `  •••start_col ≤ 30<br>	∧ -1.reserved = )<br>	∧ -2.internal_type not in {ThisExpression}<br>	∧ -5.reserved = (<br>	∧ +3.reserved = (<br>	∧ +4.reserved not in {[}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 248.` |
| 3 | `  -1.reserved = )<br>	∧ -2.internal_type not in {ThisExpression}<br>	∧ -5.reserved not in {(}<br>	∧ +3.reserved = (<br>	∧ +4.reserved not in {[}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 1146.` |
| 4 | `  -1.reserved = )<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.954. Support: 403.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.998. Support: 828.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.929. Support: 91.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ +4.roles in {STRING}<br>	∧ +5.reserved not in {,}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.958. Support: 862.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ +4.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 27686.` |
| 9 | `  -1.reserved not in {,, :}<br>	∧ -2.diff_offset ≤ 13<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.971. Support: 3158.` |
| 10 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 3403.` |
| 11 | `  •••start_col ≥ 12<br>	∧ -1.reserved = {<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 1438.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 11<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.967. Support: 836.` |
| 13 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 222.` |
| 14 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {ARITHMETIC}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 636.` |
| 15 | `  -1.reserved = [<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 97.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {ARGUMENT, ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 7000.` |
| 17 | `  -1.reserved = =<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 285.` |
| 18 | `  -1.reserved not in {(, ,, ;, =, {, }}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {[}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {FOR} and not in {ARITHMETIC}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 19 | `  -1.reserved not in {(, ,, ;, =, {, }}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {[}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {ARITHMETIC, FOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 120.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ -3.length ≤ 12<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.966. Support: 2860.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 3096.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ]<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 123.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.reserved = )<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 204.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 1651.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 100.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 343.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 192.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.length ≤ 1<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {CALL}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 111.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 163.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = [<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 295.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = [<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 91.` |
| 33 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 393.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 22764.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.352941176470589, "max_conf": 0.9981883764266968, "max_support": 27686, "min_conf": 0.9226804375648499, "min_support": 91, "num_rules": 34}}
```
</details>
