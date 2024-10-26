import i18next from "i18next";
import { initReactI18next } from "react-i18next";

const resources = {
    en: {
        translation: {
            'title': 'Yumechi\'s Color Recall Test',
            'colorspace': 'Color Space',
            'step_1_memorization': 'Step 1: Memorization',
            'step_1_instructions': 'Memorize the color below, take as much time as you need.',
            'step_2_delay': 'Step 2: Delay',
            'step_2_instructions': 'Wait for the delay to finish.',
            'step_3_recall': 'Step 3: Recall',
            'step_3_instructions': 'Select the color that you memorized. Take as much time as you need.',
            'step_4_score': 'Step 4: Results',
            'step_4_instructions': 'Your score is calculated based on how close your selection is to the target color.',
            'target_color': 'Target Color',
            'your_color': 'Your Color',
            'lower_is_better': 'lower is better',
            'unacceptable_warning': 'The current color will never be tested: ',
            'last_model': 'Last Color Space Used: ',
            'time_taken_memory': 'Time Taken to Memorize (unscored)',
            'time_taken_recall': 'Time Taken to Recall (unscored)',
            'btn_next': 'Next',
            'btn_reset': 'Reset',
            'calibration': 'Calibration',
            'calibration_toggle': 'Show/Hide Calibration Helper',
            'calibration_instructions': 'For best results please disable auto brightness and adjust your screen brightness to maximize contrast.',
            'unacceptable': (reason: string) => {
                switch (reason) {
                    case 'too_bright':
                        return 'Too Bright';
                    case 'too_dark':
                        return 'Too Dark';
                    case 'low_saturation':
                        return 'Low Saturation';
                    default:
                        return 'Unacceptable';
                }
            }
        }
    },
    zh: {
        translation: {

            'title': 'Yumechi的色彩回忆测试',
            'colorspace': '颜色空间',
            'step_1_memorization': '第一步：记忆',
            'step_1_instructions': '记住下面的颜色，需要多长时间都可以。',
            'step_2_delay': '第二步：延迟',
            'step_2_instructions': '等待延迟结束。',
            'step_3_recall': '第三步：回忆',
            'step_3_instructions': '选择您记住的颜色。需要多长时间都可以。',
            'step_4_score': '第四步：结果',
            'step_4_instructions': '您的分数是根据您的选择与目标颜色的接近程度计算的。',
            'target_color': '目标颜色',
            'your_color': '您选择的颜色',
            'lower_is_better': '分数越低越好',
            'unacceptable_warning': '当前颜色不会被测试： ',
            'last_model': '上次使用的颜色空间： ',
            'time_taken_memory': '记忆所用时间（不计分）',
            'time_taken_recall': '回忆所用时间（不计分）',
            'btn_next': '下一步',
            'btn_reset': '重置',
            'calibration': '校准',
            'calibration_toggle': '显示/隐藏校准助手',
            'calibration_instructions': '为了获得最佳效果，请关闭自动亮度并调整屏幕亮度以最大化对比度。',
            'unacceptable': (reason: string) => {
                switch (reason) {
                    case 'too_bright':
                        return '太亮';
                    case 'too_dark':
                        return '太暗';
                    case 'low_saturation':
                        return '饱和度太低';
                    default:
                        return '不可接受';
                }
            }
        }
    },
    ja: {
        translation: {
            'title': 'ゆめちの色記憶テスト',
            'colorspace': '色空間',
            'step_1_memorization': 'ステップ1：記憶',
            'step_1_instructions': '下の色を覚えてください。必要な時間をかけてください。',
            'step_2_delay': 'ステップ2：遅延',
            'step_2_instructions': '遅延が終了するのをお待ちください。',
            'step_3_recall': 'ステップ3：リコール',
            'step_3_instructions': '覚えた色を選択してください。必要な時間をかけてください。',
            'step_4_score': 'ステップ4：結果',
            'step_4_instructions': 'あなたの選択が目標色にどれだけ近いかに基づいてスコアが計算されます。',
            'target_color': '目標色',
            'your_color': 'あなたの色',
            'lower_is_better': '低いほどよい',
            'unacceptable_warning': '現在の色はテストされません： ',
            'last_model': '最後に使用した色空間： ',
            'time_taken_memory': '記憶にかかった時間（スコア対象外）',
            'time_taken_recall': 'リコールにかかった時間（スコア対象外）',
            'btn_next': '次へ',
            'btn_reset': 'リセット',
            'calibration': 'キャリブレーション',
            'calibration_toggle': 'キャリブレーションヘルパーを表示/非表示',
            'calibration_instructions': '最良の結果を得るために、自動輝度を無効にして画面輝度を調整して、コントラストを最大化してください。',
            'unacceptable': (reason: string) => {
                switch (reason) {
                    case 'too_bright':
                        return '明るすぎる';
                    case 'too_dark':
                        return '暗すぎる';
                    case 'low_saturation':
                        return '彩度が低い';
                    default:
                        return '受け入れられない';
                }
            }
        }
    }
};

i18next.use(initReactI18next).init({
    resources,
    fallbackLng: "en",
    interpolation: {
        escapeValue: false,
    },
});

const url = new URL(window.location.href);

if (url.searchParams.has('lang')) {
    i18next.changeLanguage(url.searchParams.get('lang')!);
} else if (url.pathname.endsWith('/zh.html')) {
    i18next.changeLanguage('zh');
} else if (url.pathname.endsWith('/ja.html')) {
    i18next.changeLanguage('ja');
}


export default i18next;