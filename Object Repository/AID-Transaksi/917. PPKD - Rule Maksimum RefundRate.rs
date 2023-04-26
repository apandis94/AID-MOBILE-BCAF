<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>917. PPKD - Rule Maksimum RefundRate</name>
   <tag></tag>
   <elementGuidId>954d0bdf-8ff6-4dc8-99db-c5a5ad59245b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;Year\&quot;: \&quot;${year}\&quot;,\n  \&quot;TotalKomisiDibagikan\&quot;: ${komisi_bag},\n  \&quot;TotalKomisiDicadangkan\&quot;: ${komisi_cad},\n  \&quot;SukuBungaDealer\&quot;: ${suku_bunga_dealer},\n  \&quot;RefundRatePersen\&quot;: ${percent_refund_rate},\n  \&quot;RefundRateNominal\&quot;: ${amnt_refund_rate}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6fa898f9-0104-48d7-8e47-323ef153393b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>e9f68e96-c092-401c-a337-dd7adce78ff6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/GetFormulaRuleMaksimum/RefundRate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>2e4ed30e-efcd-437d-bc15-862d6759ced9</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>'2022'</defaultValue>
      <description></description>
      <id>558645f9-ef34-443c-a128-0ab2470b47ab</id>
      <masked>false</masked>
      <name>year</name>
   </variables>
   <variables>
      <defaultValue>0.0</defaultValue>
      <description></description>
      <id>ef83ec13-5230-4432-adcb-e83bde160c1b</id>
      <masked>false</masked>
      <name>komisi_bag</name>
   </variables>
   <variables>
      <defaultValue>0.0</defaultValue>
      <description></description>
      <id>c7c0dd6e-e414-4a41-be53-fcaa801cd1d3</id>
      <masked>false</masked>
      <name>komisi_cad</name>
   </variables>
   <variables>
      <defaultValue>5.0</defaultValue>
      <description></description>
      <id>48ec5316-29dd-46b4-af42-acc3a765f125</id>
      <masked>false</masked>
      <name>suku_bunga_dealer</name>
   </variables>
   <variables>
      <defaultValue>0.0</defaultValue>
      <description></description>
      <id>5b27f30f-cda0-44d3-b477-2645b708f44d</id>
      <masked>false</masked>
      <name>percent_refund_rate</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>f389a777-6eee-421c-b21d-e2dc0306a34e</id>
      <masked>false</masked>
      <name>amnt_refund_rate</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
