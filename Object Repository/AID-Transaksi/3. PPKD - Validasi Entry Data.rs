<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3. PPKD - Validasi Entry Data</name>
   <tag></tag>
   <elementGuidId>086cf501-b4ef-4692-a0bc-c14c64534711</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoRek\&quot;: \&quot;${norek}\&quot;,\n  \&quot;NoPIN\&quot;: \&quot;${nopin}\&quot;,\n  \&quot;Year\&quot;: \&quot;${year}\&quot; // Year diambil dari hasil PPKD - Get realiasi, bukan tahun realsiasi\n}&quot;,
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
      <webElementGuid>b5e26efa-dde0-448b-91ce-a51b1a2bc8c1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>28837b42-d9bc-40c9-b4a2-7dbb9764fce0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/ValidasiEntryData</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9490713559'</defaultValue>
      <description></description>
      <id>62c15bb5-d946-47c2-b5ab-8d14e08f04a6</id>
      <masked>false</masked>
      <name>norek</name>
   </variables>
   <variables>
      <defaultValue>'001'</defaultValue>
      <description></description>
      <id>8922b6b9-ed1b-40f2-907e-2237cd5ae468</id>
      <masked>false</masked>
      <name>nopin</name>
   </variables>
   <variables>
      <defaultValue>'2022'</defaultValue>
      <description></description>
      <id>a4f64f01-dca8-4f97-88cc-7df5470e4943</id>
      <masked>false</masked>
      <name>year</name>
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
