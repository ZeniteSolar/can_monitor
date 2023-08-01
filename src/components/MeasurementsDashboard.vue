<template>
  <v-container fill-height fluid class="pa-0 ma-0">
    <v-row no-gutters>

      <!-- LEFT COLUMN -->
      <v-col class="ma-1">

        <!-- MPPT -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MPPT</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- MPPT Vi -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Vi
              </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_vi')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_vi')!.avg(), '00.0') }} {{ measurementCards.get('mcc_vi')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_vi')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_vi')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_vi')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Ii -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Ii </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_ii')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_ii')!.avg(), '00.0') }} {{ measurementCards.get('mcc_ii')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_ii')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_ii')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_ii')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Duty Cycle -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> D </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_d')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_d')!.avg(), '00.0') }} {{ measurementCards.get('mcc_d')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_d')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_d')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_d')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Vo -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Vo </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_vo')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_vo')!.avg(), '00.0') }} {{ measurementCards.get('mcc_vo')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_vo')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_vo')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_vo')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Pi -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Pi </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_pi')!.sum(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_pi')!.sum(), '0000') }} {{ measurementCards.get('mcc_pi')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_pi')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_pi')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '000') }} {{ measurementCards.get('mcc_pi')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

          </v-row>

        </v-card>

        <!-- MCB -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MCB</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- MCB Vi -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Vi
              </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcb_vi')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_vi')!.avg(), '00.0') }} {{ measurementCards.get('mcb_vi')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcb_vi')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcb_vi')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcb_vi')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MCB Io -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Io </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcb_io')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_io')!.avg(), '00.0') }} {{ measurementCards.get('mcb_io')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcb_io')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcb_io')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcb_io')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MCB Duty Cycle -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> D </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcb_d')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_d')!.avg(), '00.0') }} {{ measurementCards.get('mcb_d')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcb_d')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcb_d')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcb_d')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MCB Vo -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Vo </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcb_vo')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_vo')!.avg(), '00.0') }} {{ measurementCards.get('mcb_vo')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcb_vo')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcb_vo')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcb_vo')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MCB Po -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Po </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcb_po')!.sum(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_po')!.sum(), '000') }} {{ measurementCards.get('mcb_po')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcb_po')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcb_po')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '000') }} {{ measurementCards.get('mcb_po')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

          </v-row>

        </v-card>

        <div v-if="false">
          <!-- SYSTEM STATE -->
          <v-card class="ma-0 pa-0">
            <v-card-title class="py-0 mt-0 bg-primary font-weight-black">STATE</v-card-title>

            <!-- DATA -->
            <v-row class="px-2 ma-0">

              <!-- MAM -->
              <v-col class="ma-0 pa-1">
                <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> MAM </v-card-text>
                <v-radio-group :model-value="String(measurementCards.get('mam_machine_state')!.data[0] as number)"
                  readonly class="text-mono mx-5" style="font-size:10px">
                  <v-radio color="orange" value="0">INI</v-radio>
                  <v-radio color="orange" value="1">CON</v-radio>
                  <v-radio color="orange" value="2">IDL</v-radio>
                  <v-radio color="green" value="3">RUN</v-radio>
                  <v-radio color="red" value="4">ERR</v-radio>
                </v-radio-group>
              </v-col>

              <!-- MIC -->
              <v-col class="ma-0 pa-1">
                <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> MIC </v-card-text>
                <v-radio-group :model-value="String(measurementCards.get('mic_machine_state')!.data[0] as number)"
                  readonly class="text-mono mx-5" style="font-size:10px">
                  <v-radio color="orange" value="0">INI</v-radio>
                  <v-radio color="orange" value="1">IDL</v-radio>
                  <v-radio color="green" value="2">RUN</v-radio>
                  <v-radio color="red" value="3">ERR</v-radio>
                  <v-radio color="red" value="4">RST</v-radio>
                </v-radio-group>
              </v-col>

              <!-- MCS -->
              <v-col class="ma-0 pa-1">
                <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> MCS </v-card-text>
                <v-radio-group :model-value="String(measurementCards.get('mcs_machine_state')!.data[0] as number)"
                  readonly class="text-mono mx-5" style="font-size:10px">
                  <v-radio color="orange" value="0">INI</v-radio>
                  <v-radio color="orange" value="1">IDL</v-radio>
                  <v-radio color="green" value="2">RUN</v-radio>
                  <v-radio color="red" value="3">ERR</v-radio>
                  <v-radio color="red" value="4">RST</v-radio>
                </v-radio-group>
              </v-col>

              <!-- MAC -->
              <v-col class="ma-0 pa-1">
                <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> MAC </v-card-text>
                <v-radio-group :model-value="String(measurementCards.get('mac_machine_state')!.data[0] as number)"
                  readonly class="text-mono mx-5" style="font-size:10px">
                  <v-radio color="orange" value="0">INI</v-radio>
                  <v-radio color="orange" value="1">IDL</v-radio>
                  <v-radio color="green" value="2">RUN</v-radio>
                  <v-radio color="red" value="3">ERR</v-radio>
                  <v-radio color="red" value="4">RST</v-radio>
                </v-radio-group>
              </v-col>

              <!-- MDE -->
              <v-col class="ma-0 pa-1">
                <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> MDE </v-card-text>
                <v-radio-group :model-value="String(measurementCards.get('mde_machine_state')!.data[0] as number)"
                  readonly class="text-mono mx-5" style="font-size:10px">
                  <v-radio color="orange" value="0">INI</v-radio>
                  <v-radio color="orange" value="1">IDL</v-radio>
                  <v-radio color="green" value="2">RUN</v-radio>
                  <v-radio color="red" value="3">ERR</v-radio>
                  <v-radio color="red" value="4">RST</v-radio>
                </v-radio-group>
              </v-col>

            </v-row>

          </v-card>
        </div>

      </v-col>

      <!-- p COLUMN -->
      <v-col class="ma-1">

        <!-- BATTERY -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MAIN BAT</v-card-title>

          <!-- DATA -->

          <!-- BATTERY V -->
          <v-row class="px-2 ma-0">
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> BANK </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('bat_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_v')!.avg(), '00.00') }} {{
                      measurementCards.get('bat_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('bat_i')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_i')!.avg(), '000.0') }} {{
                      measurementCards.get('bat_i')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('bat_p')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_p')!.avg(), '00000.0') }} {{
                      measurementCards.get('bat_p')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>
          </v-row>

          <!-- BATTERY CELL -->
          <v-row class="px-2 ma-0">
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> CELL </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('bat_cell_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_cell_v')!.avg(), '00.000') }} {{
                      measurementCards.get('bat_cell_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('bat_cell_v')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('bat_cell_v')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.000') }} {{ measurementCards.get('bat_cell_v')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>

            </v-col>
          </v-row>

        </v-card>

        <!-- MOTOR -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MOTOR</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- Motor -->
            <v-col class="ma-0 pa-1">
              <div v-for="(value, index) in measurementCards.get('motor_d')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('motor_d')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('motor_d')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>

              <v-progress-linear :model-value="measurementCards.get('motor_rpm')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('motor_rpm')!.avg(), '0000.0') }} {{
                      measurementCards.get('motor_rpm')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>

          </v-row>

        </v-card>

      </v-col>

      <!-- RIGHT COLUMN -->
      <v-col class="ma-1">

        <!-- STEERING -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">STEERING</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- Steering Direction -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> D </v-card-text>

              <div v-for="(value, index) in measurementCards.get('dir_pos')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('dir_pos')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('dir_pos')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- Steering Battery -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> B </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('dir_bat_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('dir_bat_v')!.avg(), '00.00') }} {{
                      measurementCards.get('dir_bat_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('dir_bat_i')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('dir_bat_i')!.avg(), '00.00') }} {{
                      measurementCards.get('dir_bat_i')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('dir_bat_p')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('dir_bat_p')!.avg(), '000.0') }} {{
                      measurementCards.get('dir_bat_p')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>

          </v-row>

        </v-card>

        <!-- AUXILIAR BATTERIES -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">AUX BATS</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- Auxiliar Battery -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> STE </v-card-text>

              <v-progress-linear :model-value="(measurementCards.get('mcb_vo')!.data[0] as number)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_vo')!.data[0] as number, '00.00') }} {{
                      measurementCards.get('mcb_vo')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="(measurementCards.get('mcb_io')!.data[0] as number)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_io')!.data[0] as number, '00.0') }} {{
                      measurementCards.get('mcb_io')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="(measurementCards.get('mcb_po')!.data[0] as number)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_po')!.data[0] as number, '000.0') }} {{
                      measurementCards.get('mcb_po')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>

            <!-- Steering Battery -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> AUX </v-card-text>

              <v-progress-linear :model-value="(measurementCards.get('mcb_vo')!.data[1] as number)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_vo')!.data[1] as number, '00.00') }} {{
                      measurementCards.get('mcb_vo')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="(measurementCards.get('mcb_io')!.data[1] as number)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_io')!.data[1] as number, '00.0') }} {{
                      measurementCards.get('mcb_io')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="(measurementCards.get('mcb_po')!.data[1] as number)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcb_po')!.data[1] as number, '000.0') }} {{
                      measurementCards.get('mcb_po')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>

          </v-row>

        </v-card>

        <!-- SYSTEM CONTROL -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">CONTROL</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- System switches -->
            <v-col class="ma-0 pa-1">
              <v-switch :true-value="true" :model-value="(measurementCards.get('motor_on')!.data[0] as boolean)"
                label="Motor" color='red' readonly inset hide-details />

              <v-switch :true-value="true" :model-value="(measurementCards.get('boat_on')!.data[0] as boolean)"
                label="Boat" color='red' readonly inset hide-details />

              <v-switch :true-value="true" :model-value="(measurementCards.get('dms_on')!.data[0] as boolean)" label="DMS"
                color='red' readonly inset hide-details />

              <v-switch :true-value="true" :model-value="(measurementCards.get('motor_rev')!.data[0] as boolean)"
                label="REV" color='red' readonly inset hide-details />

            </v-col>

            <!-- System switches -->
            <v-col class="ma-0 pa-1">
              <v-switch v-for="(pump, index) in measurementCards.get('pump')!.data" :key="index" :true-value="true"
                :model-value="(pump as boolean)" :label="`BP ${index + 1}`" color='red' readonly inset hide-details />

            </v-col>

          </v-row>

        </v-card>

        <!-- SYSTEM ERRORS -->
        <div v-if="false">
          <v-card class="ma-0 pa-0">
            <v-card-title class="py-0 mt-0 bg-primary font-weight-black">ERRORS</v-card-title>

            <!-- DATA -->

            <!-- MAM -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MAM [{{
                measurementCards.get('mam_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 1" color="green"
                  size="x-small" style="px-2">overcurrent</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 2" color="green"
                  size="x-small" style="px-2">overvoltage</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 4" color="green"
                  size="x-small" style="px-2">overheat</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 8" color="green"
                  size="x-small" style="px-2">fault</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 16" color="green"
                  size="x-small" style="px-2">no canbus</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 16" color="green"
                  size="x-small" style="px-2">no contactor</v-chip>
              </p>
            </v-row>

            <!-- MIC -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MIC [{{
                measurementCards.get('mic_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mic_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mic_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
              </p>
            </v-row>

            <!-- MCS -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MCS [{{
                measurementCards.get('mcs_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mcs_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mcs_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
                <v-chip :hidden="(measurementCards.get('mcs_error_code')!.data[0] as number) !== 2" color="red"
                  size="x-small" style="px-2">no charge</v-chip>
              </p>
            </v-row>

            <!-- MAC -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MAC [{{
                measurementCards.get('mac_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mac_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mac_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
              </p>
            </v-row>

            <!-- MDE -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MDE [{{
                measurementCards.get('mde_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 2" color="red"
                  size="x-small" style="px-2">invalid tail</v-chip>
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 4" color="red"
                  size="x-small" style="px-2">wrong side turn</v-chip>
              </p>
            </v-row>

          </v-card>
        </div>

      </v-col>

    </v-row>

    <v-divider class="my-4"></v-divider>

    <v-card>
      Last message at: {{ last_msg_time }}
    </v-card>

    <!-- <v-card>
      <v-card-title>Can Messages</v-card-title>
      <v-card-item v-for="( message, index ) in  messages.values() " :key="index"> {{ message }}</v-card-item>
    </v-card> -->

  </v-container>
</template>

<style scoped>
.invert {
  mix-blend-mode: difference;
}

/* CUSTOM TINY SWITCH */
:deep(.v-switch__thumb) {
  height: 10px;
  width: 10px;
  offset: 1px;
}

:deep(.v-switch--inset .v-switch__track) {
  height: 14px;
  width: 34px;
  border-radius: 7px;
}

:deep(.v-switch .v-selection-control) {
  min-height: 10px;
}

:deep(.v-selection-control--density-default) {
  --v-selection-control-size: 20px
}
</style>


<script setup lang="ts">
import { Event, listen } from '@tauri-apps/api/event'
import { format } from 'numerable'
import { Ref, reactive, ref } from 'vue'
import { GenericCardData } from '../measurement_types'

// Getting data from the backend
// type CanMessage = {
//   timestamp: number,
//   message: string
// }
// const messages: Ref<CanMessage[]> = ref([])
const last_msg_time: Ref<number | null> = ref(null)
await listen('can_message', (event: Event<object>) => {
  let message = event.payload

  // Put message into the debugger
  // messages.value.unshift({ timestamp: Date.now(), message: JSON.parse(JSON.stringify(message)) } as CanMessage)
  // if (messages.value.length > 10) {
  //   messages.value.pop()
  // }

  last_msg_time.value = Date.now()

  // Get all boat data and update each card accordingly
  Object.entries(message).forEach((entry) => {
    const [key, data] = entry

    if (data !== null) {
      const card_instance = measurementCards.get(key)
      if (card_instance === null || card_instance === undefined) {
        return
      }

      if (typeof (data) === "number") {
        card_instance.data[0] = data
        return
      }

      if (typeof (data) === "boolean") {
        card_instance.data[0] = data
        return
      }

      if (Array.isArray(data)) {
        card_instance.data = data
        return
      }

    }
  })
})

const measurementCards = reactive(new Map<string, GenericCardData>)
measurementCards.set("motor_d", new GenericCardData(
  "Motor D",
  "ESC PWM Duty-Cycle",
  '%',
  0,
  100,
))
measurementCards.set("motor_rpm", new GenericCardData(
  "Motor RPM",
  "Motor RPM",
  'RPM',
  0,
  6000,
))
measurementCards.set("bat_v", new GenericCardData(
  "Bat Cell V",
  "Battery Voltage",
  'V',
  30,
  60,
))
measurementCards.set("bat_i", new GenericCardData(
  "Bat I",
  "Battery Current",
  'A',
  -200,
  200,
))
measurementCards.set("bat_p", new GenericCardData(
  "Bat Cell V",
  "Battery Power",
  'W',
  -10000,
  10000,
))
measurementCards.set("bat_cell_v", new GenericCardData(
  "Bat V",
  "Battery Voltage",
  'V',
  10,
  16,
))
measurementCards.set("dir_pos", new GenericCardData(
  "Dir H",
  "Steering System Sensors Position",
  '°',
  -135,
  135,
))
measurementCards.set("dir_bat_v", new GenericCardData(
  "Dir V",
  "Steering System Battery Voltage",
  'V',
  7,
  15,
))
measurementCards.set("dir_bat_i", new GenericCardData(
  "Dir I",
  "Steering System Battery Current",
  'A',
  0,
  20,
))
measurementCards.set("dir_bat_p", new GenericCardData(
  "Dir I",
  "Steering System Battery Power",
  'W',
  0,
  300,
))
measurementCards.set("mcb_po", new GenericCardData(
  "MCBs Po",
  "MCBs Output Power",
  'W',
  0,
  300,
))
measurementCards.set("mcb_vi", new GenericCardData(
  "MCB Vi",
  "MCB Input Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcb_io", new GenericCardData(
  "MCB Io",
  "MCB Output Current",
  'A',
  0,
  15,
))
measurementCards.set("mcb_vo", new GenericCardData(
  "MCB Vo",
  "MCB Output Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcb_d", new GenericCardData(
  "MCB D",
  "MCB Duty Cycle",
  '%',
  0,
  100,
))
measurementCards.set("mcc_pi", new GenericCardData(
  "MPPTs Pi",
  "MPPTs Input Power",
  'W',
  0,
  300,
))
measurementCards.set("mcc_vi", new GenericCardData(
  "MPPT Vi",
  "MPPT Input Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcc_ii", new GenericCardData(
  "MPPT Ii",
  "MPPT Input Current",
  'A',
  0,
  15,
))
measurementCards.set("mcc_vo", new GenericCardData(
  "MPPT Vo",
  "MPPT Output Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcc_d", new GenericCardData(
  "MPPT D",
  "MPPT Duty Cycle",
  '%',
  0,
  100,
))
measurementCards.set("pump", new GenericCardData(
  "BILDGE PUMP",
  "Bildge Pump Status",
  '',
  0,
  1,
))
measurementCards.set("boat_on", new GenericCardData(
  "BOAT ON",
  "Boat Status",
  '',
  0,
  1,
))
measurementCards.set("dms_on", new GenericCardData(
  "DMS ON",
  "Dead Man Switch Status",
  '',
  0,
  1,
))
measurementCards.set("motor_on", new GenericCardData(
  "MOTOR ON",
  "Motor Status",
  '',
  0,
  1,
))
measurementCards.set("motor_rev", new GenericCardData(
  "MOTOR REVERSE",
  "Motor Reverse Status",
  '',
  0,
  1,
))
measurementCards.set("mam_machine_state", new GenericCardData(
  "MAM MACHINE STATE",
  "MAM Machine State",
  '',
  0,
  1,
))
measurementCards.set("mic_machine_state", new GenericCardData(
  "MIC MACHINE STATE",
  "MIC Machine State",
  '',
  0,
  1,
))
measurementCards.set("mcs_machine_state", new GenericCardData(
  "MCS MACHINE STATE",
  "MCS Machine State",
  '',
  0,
  1,
))
measurementCards.set("mac_machine_state", new GenericCardData(
  "MAC MACHINE STATE",
  "MAC Machine State",
  '',
  0,
  1,
))
measurementCards.set("mde_machine_state", new GenericCardData(
  "MDE MACHINE STATE",
  "MDE Machine State",
  '',
  0,
  1,
))

measurementCards.set("mam_error_code", new GenericCardData(
  "MAM ERROR CODE",
  "MAM Error Code",
  '',
  0,
  1,
))
measurementCards.set("mic_error_code", new GenericCardData(
  "MIC ERROR CODE",
  "MIC Error Code",
  '',
  0,
  1,
))
measurementCards.set("mcs_error_code", new GenericCardData(
  "MCS ERROR CODE",
  "MCS Error Code",
  '',
  0,
  1,
))
measurementCards.set("mac_error_code", new GenericCardData(
  "MAC ERROR CODE",
  "MAC Error Code",
  '',
  0,
  1,
))
measurementCards.set("mde_error_code", new GenericCardData(
  "MDE ERROR CODE",
  "MDE Error Code",
  '',
  0,
  1,
))

</script>